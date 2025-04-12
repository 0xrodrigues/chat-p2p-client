// use std::collections::HashMap;
// use std::fs::{self, File, OpenOptions};
// use std::io::{BufReader, BufWriter};
// use std::path::PathBuf;
use base64::Engine;


use serde::{Deserialize, Serialize};
// use dirs::home_dir;

use crate::contacts::ContactBook;

/// Representação de um contato com nome e chave pública codificada
#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub pubkey: String,
}

/// Retorna o diretório base de contatos do usuário
// fn get_contacts_dir() -> PathBuf {
//     let mut base = home_dir().expect("❌ Failed to access home directory");
//     base.push(".chat-p2p");
//     base
// }

/// Retorna o caminho completo do arquivo de contatos
// fn get_contacts_path() -> PathBuf {
//     let mut path = get_contacts_dir();
//     path.push("contacts.json");
//     path
// }

// /// Carrega os contatos do arquivo (ou retorna vazio)
// pub fn load_contacts() -> HashMap<String, String> {
//     let path = get_contacts_path();
//     if !path.exists() {
//         return HashMap::new();
//     }

//     let file = File::open(path).expect("❌ Failed to open contacts file");
//     let reader = BufReader::new(file);
//     serde_json::from_reader(reader).unwrap_or_else(|_| HashMap::new())
// }

// /// Salva os contatos no arquivo
// pub fn save_contacts(contacts: &HashMap<String, String>) {
//     let path = get_contacts_path();
//     fs::create_dir_all(get_contacts_dir()).expect("❌ Failed to create contact directory");

//     let file = OpenOptions::new()
//         .create(true)
//         .write(true)
//         .truncate(true)
//         .open(path)
//         .expect("❌ Failed to save contact store");

//     let writer = BufWriter::new(file);
//     serde_json::to_writer_pretty(writer, contacts).expect("❌ Failed to write contacts");
// }

pub fn add_contact(name: &str, base64_pubkey: &str) {
    let pubkey_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_pubkey)
        .expect("❌ Invalid base64");

    let mut book = ContactBook::load();
    book.add_contact(name, &pubkey_bytes);
    book.save();

    println!("✅ Contact '{}' added!", name);
}

pub fn list_contacts() {
    let book = ContactBook::load();
    println!("📇 Your contacts:");
    for (name, pubkey) in book.contacts {
        println!("• {} => {}", name, pubkey);
    }
}

pub fn get_pubkey(name: &str) -> Option<Vec<u8>> {
    let book = ContactBook::load();
    book.get_pubkey(name)
}
