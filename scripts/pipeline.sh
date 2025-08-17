#!/bin/bash

# Pipeline Bash para API Rust Monorepo
# Uso: ./pipeline.sh [comando]

set -e  # Exit on error

# Cores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Função para mostrar mensagens coloridas
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Função para mostrar ajuda
show_help() {
    print_color $GREEN "🚀 API Rust Monorepo - Comandos disponíveis:"
    echo ""
    print_color $YELLOW "  build          - Compila o projeto em modo debug"
    print_color $YELLOW "  build-release  - Compila o projeto em modo release"
    print_color $YELLOW "  test           - Executa todos os testes"
    print_color $YELLOW "  clean          - Remove arquivos de build"
    print_color $YELLOW "  fmt            - Formata o código"
    print_color $YELLOW "  fmt-check      - Verifica formatação do código"
    print_color $YELLOW "  clippy         - Executa clippy (linter)"
    print_color $YELLOW "  check          - Executa todas as verificações"
    print_color $YELLOW "  lint           - Formata e executa clippy"
    print_color $YELLOW "  security       - Verifica vulnerabilidades"
    print_color $YELLOW "  docker-build   - Constrói imagem Docker"
    print_color $YELLOW "  docker-run     - Executa container Docker"
    print_color $YELLOW "  docker-stop    - Para container Docker"
    print_color $YELLOW "  dev            - Inicia modo desenvolvimento"
    print_color $YELLOW "  pipeline       - Executa pipeline completo"
    print_color $YELLOW "  validate       - Valida código para produção"
    print_color $YELLOW "  status         - Mostra status do projeto"
    print_color $YELLOW "  install-tools  - Instala ferramentas"
    print_color $YELLOW "  help           - Mostra esta ajuda"
    echo ""
}

# Função para verificar se o Rust está instalado
check_rust() {
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        local rust_version=$(rustc --version)
        local cargo_version=$(cargo --version)
        print_color $GREEN "✅ Rust instalado: $rust_version"
        print_color $GREEN "✅ Cargo instalado: $cargo_version"
        return 0
    else
        print_color $RED "❌ Rust não está instalado ou não está no PATH"
        print_color $YELLOW "Instale o Rust em: https://rustup.rs/"
        return 1
    fi
}

# Função para build
build_project() {
    print_color $GREEN "🔨 Compilando projeto..."
    if cargo build; then
        print_color $GREEN "✅ Compilação concluída!"
    else
        print_color $RED "❌ Erro na compilação"
        exit 1
    fi
}

# Função para build release
build_release() {
    print_color $GREEN "🚀 Compilando projeto em modo release..."
    if cargo build --release; then
        print_color $GREEN "✅ Compilação release concluída!"
    else
        print_color $RED "❌ Erro na compilação release"
        exit 1
    fi
}

# Função para testes
test_project() {
    print_color $GREEN "🧪 Executando testes..."
    if cargo test; then
        print_color $GREEN "✅ Testes concluídos!"
    else
        print_color $RED "❌ Erro nos testes"
        exit 1
    fi
}

# Função para limpeza
clean_project() {
    print_color $YELLOW "🧹 Limpando arquivos de build..."
    cargo clean
    print_color $GREEN "✅ Limpeza concluída!"
}

# Função para formatação
format_code() {
    print_color $GREEN "🎨 Formatando código..."
    if cargo fmt --all; then
        print_color $GREEN "✅ Formatação concluída!"
    else
        print_color $RED "❌ Erro na formatação"
        exit 1
    fi
}

# Função para verificar formatação
check_format() {
    print_color $GREEN "🔍 Verificando formatação..."
    if cargo fmt --all -- --check; then
        print_color $GREEN "✅ Formatação OK!"
    else
        print_color $RED "❌ Formatação incorreta"
        exit 1
    fi
}

# Função para clippy
run_clippy() {
    print_color $GREEN "🔍 Executando clippy..."
    if cargo clippy --all-targets --all-features -- -D warnings; then
        print_color $GREEN "✅ Clippy OK!"
    else
        print_color $RED "❌ Clippy encontrou problemas"
        exit 1
    fi
}

# Função para verificação de segurança
check_security() {
    print_color $GREEN "🔒 Verificando segurança..."
    if command -v cargo-audit &> /dev/null; then
        if cargo audit; then
            print_color $GREEN "✅ Verificação de segurança concluída!"
        else
            print_color $YELLOW "⚠️ Vulnerabilidades encontradas"
        fi
    else
        print_color $YELLOW "⚠️ cargo-audit não está instalado"
        print_color $YELLOW "Instale com: cargo install cargo-audit"
    fi
}

# Função para Docker
build_docker() {
    print_color $GREEN "🐳 Construindo imagem Docker..."
    if docker build -t api-rust:latest .; then
        print_color $GREEN "✅ Imagem Docker construída!"
    else
        print_color $RED "❌ Erro na construção Docker"
        exit 1
    fi
}

start_docker() {
    print_color $GREEN "🚀 Iniciando container Docker..."
    docker-compose up -d
    print_color $GREEN "✅ Container iniciado!"
}

stop_docker() {
    print_color $YELLOW "⏹️ Parando container Docker..."
    docker-compose down
    print_color $GREEN "✅ Container parado!"
}

# Função para desenvolvimento
start_dev() {
    print_color $GREEN "🔄 Iniciando modo desenvolvimento..."
    cargo run
}

# Função para pipeline completo
run_pipeline() {
    print_color $GREEN "🚀 Executando pipeline completo..."
    clean_project
    check_format
    run_clippy
    test_project
    build_release
    print_color $GREEN "🎉 Pipeline completo executado com sucesso!"
}

# Função para validação
validate_code() {
    print_color $GREEN "🎯 Validando código para produção..."
    check_format
    run_clippy
    check_security
    test_project
    build_release
    print_color $GREEN "🎯 Código validado para produção!"
}

# Função para status
show_status() {
    print_color $GREEN "📊 Status do Projeto:"
    if command -v rustc &> /dev/null; then
        local rust_version=$(rustc --version)
        local cargo_version=$(cargo --version)
        echo "Rust version: $rust_version"
        echo "Cargo version: $cargo_version"
        
        # Verificar sistema operacional
        if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            echo "Sistema: Linux"
        elif [[ "$OSTYPE" == "darwin"* ]]; then
            echo "Sistema: macOS"
        else
            echo "Sistema: $OSTYPE"
        fi
        
        print_color $GREEN "✅ Status verificado!"
    else
        print_color $RED "❌ Erro ao verificar status"
    fi
}

# Função para instalar ferramentas
install_tools() {
    print_color $GREEN "📦 Instalando ferramentas..."
    if cargo install cargo-watch && \
       cargo install cargo-tarpaulin && \
       cargo install cargo-audit && \
       cargo install cargo-deny; then
        print_color $GREEN "✅ Ferramentas instaladas!"
    else
        print_color $RED "❌ Erro ao instalar ferramentas"
    fi
}

# Main execution
print_color $GREEN "🚀 API Rust Monorepo - Pipeline Bash"
echo "============================================="

# Verificar instalação do Rust
if ! check_rust; then
    exit 1
fi

# Obter comando (padrão: help)
COMMAND=${1:-help}

# Executar comando solicitado
case $COMMAND in
    "build")
        build_project
        ;;
    "build-release")
        build_release
        ;;
    "test")
        test_project
        ;;
    "clean")
        clean_project
        ;;
    "fmt")
        format_code
        ;;
    "fmt-check")
        check_format
        ;;
    "clippy")
        run_clippy
        ;;
    "check")
        check_format
        run_clippy
        print_color $GREEN "✅ Todas as verificações passaram!"
        ;;
    "lint")
        format_code
        run_clippy
        print_color $GREEN "✅ Linting concluído!"
        ;;
    "security")
        check_security
        ;;
    "docker-build")
        build_docker
        ;;
    "docker-run")
        start_docker
        ;;
    "docker-stop")
        stop_docker
        ;;
    "dev")
        start_dev
        ;;
    "pipeline")
        run_pipeline
        ;;
    "validate")
        validate_code
        ;;
    "status")
        show_status
        ;;
    "install-tools")
        install_tools
        ;;
    "help")
        show_help
        ;;
    *)
        print_color $RED "❌ Comando desconhecido: $COMMAND"
        show_help
        exit 1
        ;;
esac
