// src/identity.rs

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use base64::{engine::general_purpose::STANDARD, Engine};
use dirs::home_dir;
use uuid::Uuid;

#[allow(dead_code)]
pub struct Identity {
    pub id: String,
    pub public_key: String,
}

impl Identity {
    pub fn init() -> Self {
        let id = Uuid::new_v4().to_string();
        let pubkey = STANDARD.encode(id.as_bytes());

        let base = get_base_path();
        fs::create_dir_all(&base).expect("Failed to create identity directory");

        let pubkey_path = base.join("public.key");

        println!("Caminho da chave pública: {}", pubkey_path.display());

        let mut file = File::create(pubkey_path).expect("Failed to save public key");
        file.write_all(pubkey.as_bytes()).expect("Failed to write public key");

        println!("🆔 Identity created: {}", &id);

        Self {
            id,
            public_key: pubkey,
        }
    }

    #[allow(dead_code)]
    pub fn load() -> Self {
        let base = get_base_path();
        let pubkey_path = base.join("public.key");
        let pubkey = fs::read_to_string(pubkey_path).expect("Failed to read public key");

        let id_bytes = base64::engine::general_purpose::STANDARD
            .decode(&pubkey)
            .expect("Failed to decode public key");

        let id = Uuid::from_slice(&id_bytes).expect("Invalid UUID").to_string();

        println!("🆔 Identity loaded: {}", &id);

        Self { id, public_key: pubkey }
    }
}

pub fn get_base_path() -> PathBuf {
    home_dir()
        .expect("Failed to find home dir")
        .join(".chat-p2p")
}
