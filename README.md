# 🛡️ Chat P2P - Projeto de Mensageria Descentralizada com Rust + Kafka + Redis

Um sistema de troca de mensagens real-time baseado em identidades criptográficas, com histórico local, suporte a mensagens offline, e fluxo peer-to-peer via WebSocket. Construído do zero com foco em privacidade, desempenho e arquitetura resiliente.

---

## 🚀 Visão Geral

O Chat P2P é um experimento de engenharia de software que simula uma troca de mensagens entre peers, sem uso de servidores centrais de estado. Cada usuário gera sua própria identidade e se conecta diretamente a um backend leve que atua como roteador, usando WebSockets, Kafka e Redis.

### 🔐 Características principais:

- Identidades únicas com chaves públicas base64.
- Comunicação via WebSocket com troca de mensagens JSON.
- Armazenamento de mensagens local por contato.
- Suporte a mensagens offline com Redis.
- CLI interativa com histórico persistente.

---

## 🧱 Stack Técnica

- 🦀 **Rust** (client CLI)
- ☕ **Java + Spring Boot** (backend WebSocket + Kafka)
- 🧵 **Kafka** (fila de mensagens)
- 🧠 **Redis** (mensagens offline)
- 📡 **WebSocket** (conexão em tempo real)

---

## 📁 Estrutura do Projeto (Client)

```bash
chat-p2p-client/
├── src/
│   ├── identity.rs         # Geração e carregamento da identidade
│   ├── ws_client.rs        # Conexão WebSocket + CLI interativo
│   ├── contact_store.rs    # Armazenamento de contatos
│   ├── message_store.rs    # Histórico local de mensagens
│   └── main.rs             # Entrypoint de comandos CLI
├── Cargo.toml
└── test_full_flow.sh       # Script para teste completo entre Alice e Bob
```

---

## ⚙️ Como usar (modo local)

### 1. Gerar identidades:
```bash
CHAT_PROFILE=alice cargo run -- init
CHAT_PROFILE=bob cargo run -- init
```

### 2. Adicionar contatos:
```bash
CHAT_PROFILE=alice cargo run -- add-contact bob <chave_publica_base64_do_bob>
CHAT_PROFILE=bob cargo run -- add-contact alice <chave_publica_base64_da_alice>
```

### 3. Iniciar chat:
```bash
CHAT_PROFILE=alice cargo run -- chat ws://localhost:8080/ws bob
CHAT_PROFILE=bob   cargo run -- chat ws://localhost:8080/ws alice
```

> 💡 As mensagens ficam salvas localmente em `~/.chat-alice/messages/bob.json` e vice-versa.

### 4. Testar fluxo offline:
Você pode iniciar Alice, enviar mensagens com Bob offline, e depois subir Bob novamente. Redis manterá as mensagens pendentes até entrega.

---

## 🧪 Teste completo automático
Execute:
```bash
./test_full_flow.sh
```
Isso:
- Gera novas identidades.
- Adiciona os contatos.
- Simula troca de mensagens com peer offline.
- Mostra histórico completo.

---

## 🧠 Filosofia
> "Privacidade não é sobre esconder algo errado. É sobre liberdade, soberania e dignidade."

Este projeto é parte de uma busca por soberania digital, inspirado por ideais cypherpunk e resistência à vigilância em massa.

---

## 📣 Contribua
Se você também acredita em privacidade por padrão, abra uma issue, fork o projeto e compartilhe sua melhoria.

---

## 🧑‍💻 Autor
**@0xrodrigues** – Software Engineer, privacy advocate & digital builder.

---

## 🛡️ Licença
MIT License. Liberdade para usar, aprender e evoluir.

