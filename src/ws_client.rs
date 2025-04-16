use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use std::io::{self, Write};
use tokio::io::AsyncBufReadExt;
use crate::contact_store;
use crate::message_store;
use base64::Engine;
use crate::identity;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::payloads::payload::{ChatPayload, ChallengeRequestPayload, ChallengeResponsePayload};
use crate::payloads::outgoing_message::OutgoingMessage;
use crate::payloads::incoming_message::IncomingMessage;

use ed25519_dalek::Verifier;
use nanoid::nanoid;

pub async fn start_interactive_chat(server_url: &str, peer: &str) {
    let my_identity = identity::Identity::load();
    let from_pub = my_identity.public_key.clone();
    let url = format!("{}?pub={}", server_url, from_pub);
    let url = Url::parse(&url).expect("❌ Invalid URL");

    println!("🌐 Connecting to {}", url);

    let (ws_stream, response) = connect_async(url).await.expect("❌ WebSocket connection failed");
    println!("🔗 WebSocket connected (status: {})", response.status());
    println!("💬 Type your messages below. Type 'exit' to quit.\n");

    let (write, mut read) = ws_stream.split();
    let write = Arc::new(Mutex::new(write));

    // 🔐 Enviar challenge-request após conexão
    let pubkey_bytes = contact_store::get_pubkey(peer).expect("❌ Unknown contact");
    let encoded_to = base64::engine::general_purpose::STANDARD.encode(pubkey_bytes);
    let nonce = nanoid!();

    let challenge_msg = OutgoingMessage::ChallengeRequest {
        from: from_pub.clone(),
        to: encoded_to.clone(),
        payload: ChallengeRequestPayload {
            nonce: nonce.clone(),
        },
    };

    let challenge_json = serde_json::to_string(&challenge_msg).expect("❌ Failed to serialize challenge-request");
    {
        let mut writer = write.lock().await;
        writer.send(Message::Text(challenge_json)).await.unwrap();
    }
    println!("🔐 Sent challenge-request to {} with nonce: {}", peer, nonce);

    let write_clone = Arc::clone(&write);
    let sent_nonce = nonce.clone();
    let peer_clone = peer.to_string();

    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                if let Ok(parsed) = serde_json::from_str::<IncomingMessage>(&text) {
                    match parsed {
                        IncomingMessage::Chat { from, payload, .. } => {
                            let dt = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
                            println!("[{}] 📥 {}: {}", dt, from, payload.msg);
                            message_store::save_message(&from, &payload.msg, true);
                        },
                        IncomingMessage::ChallengeRequest { from, payload, .. } => {
                            println!("🔐 Received challenge-request from {} with nonce: {}", from, payload.nonce);

                            let my_identity = identity::Identity::load();
                            let signature = my_identity.sign(&payload.nonce);

                            let response = OutgoingMessage::ChallengeResponse {
                                from: my_identity.public_key.clone(),
                                to: from.clone(),
                                payload: ChallengeResponsePayload {
                                    nonce: payload.nonce.clone(),
                                    signature,
                                },
                            };

                            let json = serde_json::to_string(&response).expect("❌ Failed to serialize challenge-response");
                            let mut writer = write_clone.lock().await;
                            writer.send(Message::Text(json)).await.unwrap();

                            println!("✅ Sent challenge-response to {}", from);
                        },
                        IncomingMessage::ChallengeResponse { from, payload, .. } => {
                            println!("✅ Received challenge-response from {} with signed nonce: {}", from, payload.nonce);

                            // 🔐 Verificar assinatura
                            let pubkey_bytes = base64::engine::general_purpose::STANDARD
                                .decode(&from)
                                .expect("❌ Failed to decode pubkey");

                            let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey_bytes)
                                .expect("❌ Invalid public key");

                            let sig_bytes = base64::engine::general_purpose::STANDARD
                                .decode(&payload.signature)
                                .expect("❌ Failed to decode signature");

                            let signature = ed25519_dalek::Signature::from_bytes(&sig_bytes)
                                .expect("❌ Invalid signature format");

                            if pubkey.verify(sent_nonce.as_bytes(), &signature).is_ok() {
                                println!("✅ Peer {} authenticated successfully.", from);
                            } else {
                                println!("❌ Invalid signature from {}", from);
                            }
                        },
                    }
                } else {
                    let dt = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
                    println!("[{}] ⚠️ Invalid message: {}", dt, text);
                }
            }
        }
    });

    // 🧑‍💻 Entrada do usuário
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let trimmed = line.trim();

        if trimmed == "exit" || trimmed == "quit" {
            println!("👋 Exiting chat.");
            break;
        }

        if !trimmed.is_empty() {
            let msg = OutgoingMessage::Chat {
                from: from_pub.clone(),
                to: encoded_to.clone(),
                payload: ChatPayload {
                    msg: trimmed.to_string(),
                },
            };

            let json = serde_json::to_string(&msg).expect("❌ Failed to serialize message");
            let mut writer = write.lock().await;
            writer.send(Message::Text(json)).await.unwrap();

            message_store::save_message(&peer_clone, trimmed, false);
            let dt = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
            println!("[{}] 📤 me: {}", dt, trimmed);
        }

        print!("> ");
        io::stdout().flush().unwrap();
    }
}
