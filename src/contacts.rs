// src/contacts.rs

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ContactBook {
    pub contacts: HashMap<String, String>, // name -> base64 pubkey
}

impl ContactBook {
    pub fn load() -> Self {
        let path = Self::get_path();
        if path.exists() {
            let file = File::open(path).expect("❌ Failed to open contact store");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| ContactBook {
                contacts: HashMap::new(),
            })
        } else {
            ContactBook {
                contacts: HashMap::new(),
            }
        }
    }

    pub fn save(&self) {
        let path = Self::get_path();
        fs::create_dir_all(path.parent().unwrap()).expect("❌ Failed to create contact dir");
        let file = File::create(path).expect("❌ Failed to save contact store");
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self).expect("❌ Failed to write contacts");
    }

    pub fn add_contact(&mut self, name: &str, pubkey_bytes: &[u8]) {
        let b64 = STANDARD.encode(pubkey_bytes);
        self.contacts.insert(name.to_string(), b64);
    }

    pub fn get_pubkey(&self, name: &str) -> Option<Vec<u8>> {
        self.contacts.get(name).and_then(|b64| STANDARD.decode(b64).ok())
    }

    fn get_path() -> PathBuf {
        let mut path = dirs::home_dir().expect("❌ Failed to find home directory");
        path.push(".chat-p2p/contacts.json");
        path
    }
}
