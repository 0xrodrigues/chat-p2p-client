use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::env;

use base64::{engine::general_purpose::STANDARD, Engine};
use dirs::home_dir;

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer, Signature};
use rand_core::OsRng;

pub struct Identity {
    pub public_key: String,
    keypair: Keypair,
}

impl Identity {
    pub fn init() -> Self {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let pubkey_base64 = STANDARD.encode(keypair.public.to_bytes());

        let base = get_base_path();
        fs::create_dir_all(&base).expect("❌ Failed to create identity directory");

        let pubkey_path = base.join("public.key");
        let mut pub_file = File::create(pubkey_path).expect("❌ Failed to save public key");
        pub_file
            .write_all(pubkey_base64.as_bytes())
            .expect("❌ Failed to write public key");

        let privkey_path = base.join("private.key");
        let mut priv_file = File::create(privkey_path).expect("❌ Failed to save private key");
        priv_file
            .write_all(&keypair.secret.to_bytes())
            .expect("❌ Failed to write private key");

        println!("🆔 Identity created");
        println!("🔑 Public Key (base64): {}", pubkey_base64);

        Self {
            public_key: pubkey_base64,
            keypair,
        }
    }

    pub fn load() -> Self {
        let base = get_base_path();

        let pubkey_str = fs::read_to_string(base.join("public.key"))
            .expect("❌ Failed to read public key")
            .trim()
            .to_string();
        let pubkey_bytes = STANDARD
            .decode(&pubkey_str)
            .expect("❌ Failed to decode public key");
        let pubkey = PublicKey::from_bytes(&pubkey_bytes).expect("❌ Invalid public key");

        let privkey_bytes = fs::read(base.join("private.key")).expect("❌ Failed to read private key");
        let secret = SecretKey::from_bytes(&privkey_bytes).expect("❌ Invalid private key");

        let keypair = Keypair { secret, public: pubkey };

        println!("🆔 Identity loaded");
        println!("🔑 Public Key (base64): {}", pubkey_str);

        Self {
            public_key: pubkey_str,
            keypair,
        }
    }

    pub fn export_pubkey() {
        let profile = std::env::var("CHAT_PROFILE").unwrap_or_else(|_| "p2p".to_string());
        let base = get_base_path();
        let pubkey_path = base.join("public.key");

        let pubkey = fs::read_to_string(&pubkey_path).expect("❌ Failed to read public key");

        let export_path = format!("{}.pub", profile);
        let mut file = File::create(&export_path).expect("❌ Failed to create export file");

        file.write_all(pubkey.as_bytes())
            .expect("❌ Failed to write public key");

        println!("✅ Public key exported to '{}'", export_path);
    }

    pub fn sign(&self, nonce: &str) -> String {
        let signature: Signature = self.keypair.sign(nonce.as_bytes());
        STANDARD.encode(signature.to_bytes())
    }
}

pub fn get_base_path() -> PathBuf {
    let profile = env::var("CHAT_PROFILE").unwrap_or_else(|_| "p2p".to_string());
    home_dir()
        .expect("⚠️ Could not access home directory")
        .join(format!(".chat-{}", profile))
}
