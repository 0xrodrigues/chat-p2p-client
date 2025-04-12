mod identity;
mod ws_client;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "init" => {
            identity::Identity::init();
        }
        "ws" => {
            let url = args.get(2).expect("❌ Missing WebSocket URL");
            ws_client::start_interactive_client(url).await;
        }
        _ => {
            println!("Commands:");
            println!("  init                   - Generate new identity");
            println!("  ws <url>               - Connect to WebSocket and chat");
        }
    }
}
