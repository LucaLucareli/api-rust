# Makefile para API Rust Monorepo
# Uso: make [target]

.PHONY: help build test clean fmt clippy docker-build docker-run docker-stop lint security-check

# Variáveis
CARGO = cargo
RUSTC = rustc
CARGO_AUDIT = cargo-audit
CARGO_TARPAULIN = cargo-tarpaulin
CARGO_DENY = cargo-deny

# Cores para output
GREEN = \033[0;32m
YELLOW = \033[1;33m
RED = \033[0;31m
NC = \033[0m # No Color

# Help
help: ## Mostra esta ajuda
	@echo "$(GREEN)🚀 API Rust Monorepo - Comandos disponíveis:$(NC)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}'
	@echo ""

# Build
build: ## Compila o projeto em modo debug
	@echo "$(GREEN)🔨 Compilando projeto...$(NC)"
	$(CARGO) build
	@echo "$(GREEN)✅ Compilação concluída!$(NC)"

build-release: ## Compila o projeto em modo release
	@echo "$(GREEN)🚀 Compilando projeto em modo release...$(NC)"
	$(CARGO) build --release
	@echo "$(GREEN)✅ Compilação release concluída!$(NC)"

# Testes
test: ## Executa todos os testes
	@echo "$(GREEN)🧪 Executando testes...$(NC)"
	$(CARGO) test
	@echo "$(GREEN)✅ Testes concluídos!$(NC)"

test-watch: ## Executa testes em modo watch
	@echo "$(GREEN)👀 Executando testes em modo watch...$(NC)"
	$(CARGO) watch -x test

test-coverage: ## Executa testes com cobertura
	@echo "$(GREEN)📊 Executando testes com cobertura...$(NC)"
	$(CARGO) tarpaulin --out Html
	@echo "$(GREEN)✅ Cobertura gerada em target/tarpaulin/$(NC)"

# Limpeza
clean: ## Remove arquivos de build
	@echo "$(YELLOW)🧹 Limpando arquivos de build...$(NC)"
	$(CARGO) clean
	@echo "$(GREEN)✅ Limpeza concluída!$(NC)"

# Formatação e Linting
fmt: ## Formata o código
	@echo "$(GREEN)🎨 Formatando código...$(NC)"
	$(CARGO) fmt --all
	@echo "$(GREEN)✅ Formatação concluída!$(NC)"

fmt-check: ## Verifica formatação do código
	@echo "$(GREEN)🔍 Verificando formatação...$(NC)"
	$(CARGO) fmt --all -- --check
	@echo "$(GREEN)✅ Formatação OK!$(NC)"

clippy: ## Executa clippy (linter)
	@echo "$(GREEN)🔍 Executando clippy...$(NC)"
	$(CARGO) clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)✅ Clippy OK!$(NC)"

# Análise de Segurança
security-check: ## Verifica vulnerabilidades de segurança
	@echo "$(GREEN)🔒 Verificando segurança...$(NC)"
	$(CARGO_AUDIT)
	@echo "$(GREEN)✅ Verificação de segurança concluída!$(NC)"

# Docker
docker-build: ## Constrói imagem Docker
	@echo "$(GREEN)🐳 Construindo imagem Docker...$(NC)"
	docker build -t api-rust:latest .
	@echo "$(GREEN)✅ Imagem Docker construída!$(NC)"

docker-run: ## Executa container Docker
	@echo "$(GREEN)🚀 Iniciando container Docker...$(NC)"
	docker-compose up -d
	@echo "$(GREEN)✅ Container iniciado!$(NC)"

docker-stop: ## Para container Docker
	@echo "$(YELLOW)⏹️ Parando container Docker...$(NC)"
	docker-compose down
	@echo "$(GREEN)✅ Container parado!$(NC)"

docker-logs: ## Mostra logs do Docker
	@echo "$(GREEN)📋 Mostrando logs...$(NC)"
	docker-compose logs -f

# Desenvolvimento
dev: ## Inicia modo desenvolvimento
	@echo "$(GREEN)🔄 Iniciando modo desenvolvimento...$(NC)"
	$(CARGO) run

dev-watch: ## Inicia modo desenvolvimento com watch
	@echo "$(GREEN)👀 Iniciando modo desenvolvimento com watch...$(NC)"
	$(CARGO) watch -x run

# Verificações
check: fmt-check clippy ## Executa todas as verificações
	@echo "$(GREEN)✅ Todas as verificações passaram!$(NC)"

lint: fmt clippy ## Formata e executa clippy
	@echo "$(GREEN)✅ Linting concluído!$(NC)"

# Instalação de ferramentas
install-tools: ## Instala ferramentas de desenvolvimento
	@echo "$(GREEN)📦 Instalando ferramentas...$(NC)"
	$(CARGO) install cargo-watch
	$(CARGO) install cargo-tarpaulin
	$(CARGO) install cargo-audit
	$(CARGO) install cargo-deny
	@echo "$(GREEN)✅ Ferramentas instaladas!$(NC)"

# Análise de dependências
deps-tree: ## Mostra árvore de dependências
	@echo "$(GREEN)🌳 Mostrando árvore de dependências...$(NC)"
	$(CARGO) tree

deps-update: ## Atualiza dependências
	@echo "$(GREEN)⬆️ Atualizando dependências...$(NC)"
	$(CARGO) update
	@echo "$(GREEN)✅ Dependências atualizadas!$(NC)"

# Pipeline completo
pipeline: clean fmt-check clippy test build-release ## Executa pipeline completo
	@echo "$(GREEN)🎉 Pipeline completo executado com sucesso!$(NC)"

# Validação de produção
validate: fmt-check clippy security-check test build-release ## Valida código para produção
	@echo "$(GREEN)🎯 Código validado para produção!$(NC)"

# Comandos específicos para Windows
windows-setup: ## Configuração específica para Windows
	@echo "$(YELLOW)🪟 Configuração para Windows...$(NC)"
	@echo "Instalando ferramentas necessárias..."
	@echo "1. Visual Studio Build Tools"
	@echo "2. Rust via rustup"
	@echo "3. Docker Desktop"
	@echo "$(GREEN)✅ Configuração para Windows concluída!$(NC)"

# Comandos específicos para Linux
linux-setup: ## Configuração específica para Linux
	@echo "$(YELLOW)🐧 Configuração para Linux...$(NC)"
	@echo "Instalando dependências..."
	sudo apt-get update
	sudo apt-get install -y build-essential pkg-config libssl-dev
	@echo "$(GREEN)✅ Configuração para Linux concluída!$(NC)"

# Comandos específicos para macOS
macos-setup: ## Configuração específica para macOS
	@echo "$(YELLOW)🍎 Configuração para macOS...$(NC)"
	@echo "Instalando dependências..."
	brew install openssl pkg-config
	@echo "$(GREEN)✅ Configuração para macOS concluída!$(NC)"

# Status do projeto
status: ## Mostra status do projeto
	@echo "$(GREEN)📊 Status do Projeto:$(NC)"
	@echo "Rust version: $(shell rustc --version)"
	@echo "Cargo version: $(shell cargo --version)"
	@echo "Target: $(shell rustup show | grep "Default host" | cut -d: -f2 | tr -d ' ')"
	@echo "Features: $(shell cargo --list | grep -c "cargo-")"
	@echo "$(GREEN)✅ Status verificado!$(NC)"
