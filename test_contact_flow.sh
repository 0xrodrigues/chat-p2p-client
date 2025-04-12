#!/bin/bash

set -e

echo "🧹 Limpando dados antigos..."
rm -rf ~/.chat-alice
rm -rf ~/.chat-bob

echo "🚀 Criando identidade de Alice..."
CHAT_PROFILE=alice cargo run --quiet -- init <<<$'senha123\nsenha123\n'

echo "🚀 Criando identidade de Bob..."
CHAT_PROFILE=bob cargo run --quiet -- init <<<$'senha123\nsenha123\n'

echo "🔐 Extraindo chave pública de Bob..."
BOB_PUBKEY=$(base64 ~/.chat-bob/public.key)

echo "➕ Adicionando Bob como contato de Alice..."
CHAT_PROFILE=alice cargo run --quiet -- add-contact bob "$BOB_PUBKEY"

echo "📋 Listando contatos de Alice:"
CHAT_PROFILE=alice cargo run --quiet -- list-contacts
