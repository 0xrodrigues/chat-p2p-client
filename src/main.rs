mod identity;
mod ws_client;
mod message_store;
mod contact_store;
mod contacts;

mod payloads;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "init" => {
            identity::Identity::init();
        }
        "chat" => {
            let url = args.get(2).expect("❌ Missing WebSocket URL");
            let peer = args.get(3).expect("❌ Missing peer name");

            message_store::print_history(peer);
            ws_client::start_interactive_chat(url, peer).await;
        }
        "add-contact" => {
            let name = args.get(2).expect("❌ Missing contact name");
            let key = args.get(3).expect("❌ Missing base64 pubkey");
            contact_store::add_contact(name, key);
        }
        "list-contacts" => {
            contact_store::list_contacts();
        }
        "export-pub" => {
            identity::Identity::export_pubkey();
        }
        _ => {
            println!("Commands:");
            println!("  init                          - Generate new identity");
            println!("  chat <url> <peer>             - Connect to chat and load message history");
            println!("  add-contact <name> <pubkey>   - Add new contact");
            println!("  list-contacts                 - List all contacts");
        }
    }
}
