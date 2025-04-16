use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::collections::VecDeque;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MessageEntry {
    pub content: String,
    pub timestamp: i64,
    pub incoming: bool,
}

pub fn get_store_path(peer: &str) -> PathBuf {
    let mut base = dirs::home_dir().expect("⚠️ Failed to access home directory");
    base.push(".chat-p2p/messages");
    fs::create_dir_all(&base).expect("❌ Could not create message storage directory");
    base.push(format!("{}.json", peer));
    base
}


pub fn save_message(peer: &str, content: &str, incoming: bool) {
    // Caminho completo até o arquivo de mensagens
    let path = get_store_path(peer);

    // Garante que o diretório existe
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir).expect("❌ Failed to create message directory");
    }

    // Carrega mensagens anteriores
    let mut messages = load_messages(peer);

    // Cria a nova entrada
    let now = Utc::now().timestamp();
    messages.push_back(MessageEntry {
        content: content.to_string(),
        timestamp: now,
        incoming,
    });

    // Salva sobrescrevendo o arquivo com todas as mensagens
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("❌ Failed to open message file");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &messages).expect("❌ Failed to write messages");
}

pub fn load_messages(peer: &str) -> VecDeque<MessageEntry> {
    let path = get_store_path(peer);
    if !path.exists() {
        return VecDeque::new();
    }

    let file = File::open(path).expect("❌ Failed to open message file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| VecDeque::new())
}

pub fn print_history(peer: &str) {
    let messages = load_messages(peer);
    for msg in messages {
        let dt = format_timestamp(msg.timestamp);
        let direction = if msg.incoming { "📥" } else { "📤" };
        println!("[{}] {} {}", dt, direction, msg.content);
    }
}

fn format_timestamp(ts: i64) -> String {
    match DateTime::from_timestamp(ts, 0) {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "Invalid timestamp".to_string(),
    }
}
