#!/bin/bash

set -euo pipefail

echo "🧹 Limpando perfis anteriores..."
rm -rf ~/.chat-alice ~/.chat-bob

echo "🔐 Criando identidade de Alice..."
CHAT_PROFILE=alice cargo run --quiet -- init

echo "🔐 Criando identidade de Bob..."
CHAT_PROFILE=bob cargo run --quiet -- init

echo "🔗 Adicionando Bob nos contatos de Alice..."
BOB_KEY=$(cat ~/.chat-bob/public.key)
CHAT_PROFILE=alice cargo run --quiet -- add-contact bob "$BOB_KEY"

echo "🔗 Adicionando Alice nos contatos de Bob..."
ALICE_KEY=$(cat ~/.chat-alice/public.key)
CHAT_PROFILE=bob cargo run --quiet -- add-contact alice "$ALICE_KEY"

echo
echo "📨 Enviando mensagem com Bob offline..."
echo -e "Mensagem secreta para o Bob\nexit" | CHAT_PROFILE=alice cargo run --quiet -- chat ws://localhost:8080/ws bob

echo
echo "💤 Bob está offline. Mensagem deve ter sido salva no Redis ou ignorada, dependendo da infra."
echo "🔄 Pressione ENTER para simular Bob conectando-se..."
read

echo
echo "📡 Bob conectando ao WebSocket e lendo mensagem pendente:"
echo "exit" | CHAT_PROFILE=bob cargo run --quiet -- chat ws://localhost:8080/ws alice

echo
echo "📜 Histórico de Alice:"
echo "exit" | CHAT_PROFILE=alice cargo run --quiet -- chat ws://localhost:8080/ws bob

echo
echo "✅ Fluxo completo finalizado!"