#!/bin/bash

# Script de desenvolvimento para o monorepo Rust
# Este script facilita o desenvolvimento e execução das APIs

set -e

echo "🚀 Script de desenvolvimento para o monorepo Rust"
echo ""

# Função para mostrar ajuda
show_help() {
    echo "Uso: $0 [COMANDO]"
    echo ""
    echo "Comandos disponíveis:"
    echo "  build       - Compila todas as APIs"
    echo "  run         - Executa a API principal"
    echo "  run-auth    - Executa apenas a Auth API"
    echo "  run-admin   - Executa apenas a Admin API"
    echo "  run-viewer  - Executa apenas a Viewer API"
    echo "  test        - Executa todos os testes"
    echo "  check       - Verifica dependências"
    echo "  clean       - Limpa builds"
    echo "  docker      - Inicia serviços Docker (SQL Server, Redis, Azurite)"
    echo "  migrate     - Executa migrações"
    echo "  help        - Mostra esta ajuda"
    echo ""
}

# Função para executar migrações
run_migrations() {
    echo "🗄️  Executando migrações..."
    if [ -f "./run_migrations_on_docker.sh" ]; then
        chmod +x ./run_migrations_on_docker.sh
        ./run_migrations_on_docker.sh
    else
        echo "❌ Script de migrações não encontrado"
        exit 1
    fi
}

# Função para iniciar Docker
start_docker() {
    echo "🐳 Iniciando serviços Docker..."
    if [ -f "./docker-compose.yml" ]; then
        echo "📦 Serviços disponíveis:"
        echo "   - SQL Server (Porta: 1433)"
        echo "   - Redis (Porta: 27002)"
        echo "   - Azurite (Portas: 10000, 10001, 10002)"
        echo ""
        
        docker-compose up -d
        echo "✅ Serviços Docker iniciados"
        echo "⏳ Aguardando serviços ficarem saudáveis..."
        sleep 15
        docker-compose ps
        
        echo ""
        echo "🌐 URLs dos serviços:"
        echo "   - SQL Server: localhost:1433"
        echo "   - Redis: localhost:27002"
        echo "   - Azurite Blob: localhost:10000"
        echo "   - Azurite Queue: localhost:10001"
        echo "   - Azurite Table: localhost:10002"
    else
        echo "❌ docker-compose.yml não encontrado"
        exit 1
    fi
}

# Função para executar API específica
run_api() {
    local api_name=$1
    local port=$2
    
    echo "🚀 Executando $api_name na porta $port..."
    cd "src/apps/$api_name"
    cargo run --bin $api_name &
    cd ../../..
    echo "✅ $api_name iniciada em http://localhost:$port"
}

# Processar comandos
case "${1:-help}" in
    "build")
        echo "🔨 Compilando todas as APIs..."
        cargo build --workspace
        echo "✅ Compilação concluída!"
        ;;
    
    "run")
        echo "🚀 Executando API principal..."
        cargo run
        ;;
    
    "run-auth")
        run_api "auth_api" "3001"
        ;;
    
    "run-admin")
        run_api "admin_api" "3002"
        ;;
    
    "run-viewer")
        run_api "viewer_api" "3003"
        ;;
    
    "test")
        echo "🧪 Executando todos os testes..."
        cargo test --workspace
        ;;
    
    "check")
        echo "📦 Verificando dependências..."
        cargo check --workspace
        ;;
    
    "clean")
        echo "🧹 Limpando builds..."
        cargo clean
        ;;
    
    "docker")
        start_docker
        ;;
    
    "migrate")
        run_migrations
        ;;
    
    "help"|*)
        show_help
        ;;
esac
