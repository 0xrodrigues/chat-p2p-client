use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::env;

use base64::{engine::general_purpose::STANDARD, Engine};
use dirs::home_dir;
use uuid::Uuid;

use std::fs::OpenOptions;


#[allow(dead_code)]
pub struct Identity {
    pub id: String,
    pub public_key: String,
}

impl Identity {
    pub fn init() -> Self {
        let id = Uuid::new_v4();
        let pubkey = STANDARD.encode(id.as_bytes());

        let base = get_base_path();
        fs::create_dir_all(&base).expect("Failed to create identity directory");

        let pubkey_path = base.join("public.key");

        println!("Caminho da chave pública: {}", pubkey_path.display());

        let mut file = File::create(pubkey_path).expect("Failed to save public key");
        file.write_all(pubkey.as_bytes()).expect("Failed to write public key");

        println!("🆔 Identity created (UUID): {}", id);
        println!("🔑 Public Key (base64): {}", pubkey);

        Self {
            id: id.to_string(),
            public_key: pubkey,
        }
    }

    #[allow(dead_code)]
    pub fn load() -> Self {
        let base = get_base_path();
        let pubkey_path = base.join("public.key");

        let pubkey = fs::read_to_string(pubkey_path)
            .expect("Failed to read public key")
            .trim()
            .to_string();

        let id_bytes = base64::engine::general_purpose::STANDARD
            .decode(&pubkey)
            .expect("Failed to decode public key");
        
        let id = Uuid::from_slice(&id_bytes).expect("Invalid UUID").to_string();

        println!("🆔 Identity loaded: {}", id);
        println!("🔑 Public Key (base64): {}", pubkey);

        Self { id, public_key: pubkey }
    }

    pub fn export_pubkey() {
        let profile = std::env::var("CHAT_PROFILE").unwrap_or_else(|_| "p2p".to_string());
        let base = crate::identity::get_base_path();
        let pubkey_path = base.join("public.key");

        let pubkey = std::fs::read_to_string(&pubkey_path).expect("❌ Failed to read public key");

        let export_path = format!("{}.pub", profile);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&export_path)
            .expect("❌ Failed to create export file");

        file.write_all(pubkey.as_bytes()).expect("❌ Failed to write public key");

        println!("✅ Public key exported to '{}'", export_path);
    }
}

pub fn get_base_path() -> PathBuf {
    let profile = env::var("CHAT_PROFILE").unwrap_or_else(|_| "p2p".to_string());
    home_dir()
        .expect("⚠️ Could not access home directory")
        .join(format!(".chat-{}", profile))
}
