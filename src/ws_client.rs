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


pub async fn start_interactive_chat(server_url: &str, peer: &str) {
    let my_identity = identity::Identity::load();
    let pubkey = my_identity.public_key;
    let url = format!("{}?pub={}", server_url, pubkey);
    let url = Url::parse(&url).expect("❌ Invalid URL");
    
    println!("🌐 Connecting to {}", url);

    let (ws_stream, response) = connect_async(url).await.expect("❌ WebSocket connection failed");
    println!("🔗 WebSocket connected (status: {})", response.status());
    println!("💬 Type your messages below. Type 'exit' to quit.\n");

    let (mut write, mut read) = ws_stream.split();

    let peer_clone = peer.to_string(); // para mover para a task
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                println!("📨 Received: {}", text);
                message_store::save_message(&peer_clone, &text, true);
            }
        }
    });

    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let trimmed = line.trim();
        if trimmed == "exit" || trimmed == "quit" {
            println!("👋 Exiting chat.");
            break;
        }
        if !trimmed.is_empty() {
            let pubkey_bytes = contact_store::get_pubkey(peer).expect("❌ Unknown contact");
            let encoded_to = base64::engine::general_purpose::STANDARD.encode(pubkey_bytes);
            
            let payload = serde_json::json!({
                "to": encoded_to,
                "msg": trimmed,
            });
            write.send(Message::Text(payload.to_string())).await.unwrap();
            message_store::save_message(peer, trimmed, false);
        }
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
