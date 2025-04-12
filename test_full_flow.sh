#!/bin/bash

set -e

ALICE_PUBKEY=""
BOB_PUBKEY=""

echo "🧹 Limpando perfis anteriores..."
rm -rf ~/.chat-alice
rm -rf ~/.chat-bob

echo "🔐 Criando identidade de Alice..."
CHAT_PROFILE=alice cargo run --quiet -- init
ALICE_PUBKEY=$(cat ~/.chat-alice/public.key)

echo "🔐 Criando identidade de Bob..."
CHAT_PROFILE=bob cargo run --quiet -- init
BOB_PUBKEY=$(cat ~/.chat-bob/public.key)

echo "🔗 Adicionando Bob nos contatos de Alice..."
CHAT_PROFILE=alice cargo run --quiet -- add-contact bob "$BOB_PUBKEY"

echo "🔗 Adicionando Alice nos contatos de Bob..."
CHAT_PROFILE=bob cargo run --quiet -- add-contact alice "$ALICE_PUBKEY"

echo -e "\n📨 Enviando mensagem com Bob offline..."
CHAT_PROFILE=alice cargo run --quiet -- chat ws://localhost:8080/ws bob <<EOF
Mensagem secreta para o Bob
exit
EOF

echo -e "\n💤 Bob está offline. Mensagem deve ter sido salva no Redis ou ignorada, dependendo da infra.\n"
read -p "🔄 Pressione ENTER para simular Bob conectando-se..."

echo -e "\n📡 Bob conectando ao WebSocket e lendo mensagem pendente:\n"
CHAT_PROFILE=bob cargo run --quiet -- chat ws://localhost:8080/ws alice <<EOF
exit
EOF

echo -e "\n📜 Histórico de Alice:"
CHAT_PROFILE=alice cargo run --quiet -- chat ws://localhost:8080/ws bob <<EOF
exit
EOF
