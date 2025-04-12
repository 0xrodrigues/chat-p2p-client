use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use std::io::{stdin, stdout, Write};

pub async fn start_interactive_client(server_url: &str) {
    let url = Url::parse(server_url).expect("Invalid URL");
    println!("🌐 Connecting to {}", url);

    let (ws_stream, response) = connect_async(url).await.expect("Failed to connect");
    println!("🔗 Connected with status: {}", response.status());

    let (mut write, mut read) = ws_stream.split();

    // Task to print incoming messages
    let reader_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => println!("📨 Received: {}", text),
                Ok(_) => println!("📨 Received non-text message"),
                Err(e) => {
                    eprintln!("❌ WebSocket error: {}", e);
                    break;
                }
            }
        }
    });

    // Main thread: sending messages interactively
    println!("💬 Type your message and hit Enter. Type `exit` to quit.");
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed = input.trim();

        if trimmed == "exit" {
            println!("👋 Disconnecting...");
            break;
        }

        write.send(Message::Text(trimmed.to_string())).await.unwrap();
    }

    reader_task.abort();
}
