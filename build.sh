#!/bin/bash

# Script de build para o monorepo Rust
# Este script compila todas as APIs e o projeto principal

set -e

echo "🚀 Iniciando build do monorepo Rust..."

# Verificar se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Erro: Rust não está instalado ou não está no PATH."
    echo "Instale o Rust em: https://rustup.rs/"
    exit 1
fi

echo "✅ Rust encontrado: $(cargo --version)"

# Limpar builds anteriores
echo "🧹 Limpando builds anteriores..."
cargo clean

# Verificar dependências
echo "📦 Verificando dependências..."
cargo check --workspace

# Build de desenvolvimento
echo "🔨 Compilando em modo desenvolvimento..."
cargo build --workspace

# Build de produção
echo "🚀 Compilando em modo produção..."
cargo build --release --workspace

echo "✅ Build concluído com sucesso!"
echo ""
echo "📁 Binários disponíveis em:"
echo "   - Desenvolvimento: target/debug/"
echo "   - Produção: target/release/"
echo ""
echo "🎯 Para executar:"
echo "   - Desenvolvimento: cargo run"
echo "   - Produção: ./target/release/api_rust"
