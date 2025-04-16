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

use crate::payloads::payload::{ChatPayload, ChallengeRequestPayload};
use crate::payloads::outgoing_message::OutgoingMessage;

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

    let (mut write, mut read) = ws_stream.split();

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
    write.send(Message::Text(challenge_json)).await.unwrap();
    println!("🔐 Sent challenge-request to {} with nonce: {}", peer, nonce);

    // 📥 Task para escutar mensagens recebidas
    let peer_clone = peer.to_string();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                let dt = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
                if let Ok(json_msg) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(content) = json_msg.get("msg").and_then(|v| v.as_str()) {
                        println!("[{}] 📥 {}: {}", dt, peer_clone, content);
                        message_store::save_message(&peer_clone, content, true);
                    } else {
                        println!("[{}] ⚠️ JSON message without 'msg' field: {}", dt, text);
                    }
                } else {
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
            println!("DEBUG JSON: {}", json);

            write.send(Message::Text(json)).await.unwrap();
            message_store::save_message(peer, trimmed, false);

            let dt = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
            println!("[{}] 📤 me: {}", dt, trimmed);
        }

        print!("> ");
        io::stdout().flush().unwrap();
    }
}
