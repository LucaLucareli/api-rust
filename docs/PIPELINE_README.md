# 🚀 Pipeline de Build - API Rust Monorepo

Este documento descreve os pipelines de build implementados para automatizar o desenvolvimento, testes e deploy da API Rust.

## 📋 **Pipelines Disponíveis**

### **1. GitHub Actions (CI/CD Automático)**
- **Arquivo**: `.github/workflows/rust.yml`
- **Trigger**: Push/Pull Request para `main` e `develop`
- **Execução**: Automática no GitHub

### **2. Makefile (Linux/macOS)**
- **Arquivo**: `Makefile`
- **Uso**: `make [comando]`
- **Execução**: Local

### **3. PowerShell (Windows)**
- **Arquivo**: `pipeline.ps1`
- **Uso**: `.\pipeline.ps1 [comando]`
- **Execução**: Local no Windows

### **4. Bash (Linux/macOS)**
- **Arquivo**: `pipeline.sh`
- **Uso**: `./pipeline.sh [comando]`
- **Execução**: Local no Linux/macOS

## 🔄 **Comandos Disponíveis**

| Comando | Descrição | GitHub Actions | Make | PowerShell | Bash |
|---------|-----------|----------------|------|------------|------|
| `build` | Compila em modo debug | ✅ | ✅ | ✅ | ✅ |
| `build-release` | Compila em modo release | ✅ | ✅ | ✅ | ✅ |
| `test` | Executa todos os testes | ✅ | ✅ | ✅ | ✅ |
| `clean` | Remove arquivos de build | ✅ | ✅ | ✅ | ✅ |
| `fmt` | Formata o código | ✅ | ✅ | ✅ | ✅ |
| `fmt-check` | Verifica formatação | ✅ | ✅ | ✅ | ✅ |
| `clippy` | Executa linter | ✅ | ✅ | ✅ | ✅ |
| `check` | Todas as verificações | ✅ | ✅ | ✅ | ✅ |
| `lint` | Formata + clippy | ✅ | ✅ | ✅ | ✅ |
| `security` | Verifica vulnerabilidades | ✅ | ✅ | ✅ | ✅ |
| `docker-build` | Constrói Docker | ✅ | ✅ | ✅ | ✅ |
| `docker-run` | Executa Docker | ✅ | ✅ | ✅ | ✅ |
| `docker-stop` | Para Docker | ✅ | ✅ | ✅ | ✅ |
| `dev` | Modo desenvolvimento | ❌ | ✅ | ✅ | ✅ |
| `pipeline` | Pipeline completo | ❌ | ✅ | ✅ | ✅ |
| `validate` | Validação produção | ❌ | ✅ | ✅ | ✅ |
| `status` | Status do projeto | ❌ | ✅ | ✅ | ✅ |
| `install-tools` | Instala ferramentas | ❌ | ✅ | ✅ | ✅ |

## 🎯 **Como Usar**

### **GitHub Actions (Automático)**
```bash
# Apenas faça push para main ou develop
git push origin main
# O pipeline executa automaticamente
```

### **Makefile (Linux/macOS)**
```bash
# Ver comandos disponíveis
make help

# Pipeline completo
make pipeline

# Validação para produção
make validate

# Apenas build
make build-release
```

### **PowerShell (Windows)**
```powershell
# Ver comandos disponíveis
.\pipeline.ps1 help

# Pipeline completo
.\pipeline.ps1 pipeline

# Validação para produção
.\pipeline.ps1 validate

# Apenas build
.\pipeline.ps1 build-release
```

### **Bash (Linux/macOS)**
```bash
# Ver comandos disponíveis
./pipeline.sh help

# Pipeline completo
./pipeline.sh pipeline

# Validação para produção
./pipeline.sh validate

# Apenas build
./pipeline.sh build-release
```

## 🏗️ **Pipeline Completo**

O pipeline completo executa as seguintes etapas em sequência:

1. **🧹 Clean** - Remove arquivos de build anteriores
2. **🔍 Format Check** - Verifica formatação do código
3. **🔍 Clippy** - Executa linter Rust
4. **🧪 Test** - Executa todos os testes
5. **🚀 Build Release** - Compila em modo release

### **Comando para Pipeline Completo**
```bash
# Linux/macOS
make pipeline
./pipeline.sh pipeline

# Windows
.\pipeline.ps1 pipeline
```

## 🎯 **Validação para Produção**

A validação para produção executa todas as verificações necessárias:

1. **🔍 Format Check** - Verifica formatação
2. **🔍 Clippy** - Executa linter
3. **🔒 Security** - Verifica vulnerabilidades
4. **🧪 Test** - Executa testes
5. **🚀 Build Release** - Compila release

### **Comando para Validação**
```bash
# Linux/macOS
make validate
./pipeline.sh validate

# Windows
.\pipeline.ps1 validate
```

## 🐳 **Docker Pipeline**

### **Construir Imagem**
```bash
# Linux/macOS
make docker-build
./pipeline.sh docker-build

# Windows
.\pipeline.ps1 docker-build
```

### **Executar Container**
```bash
# Linux/macOS
make docker-run
./pipeline.sh docker-run

# Windows
.\pipeline.ps1 docker-run
```

### **Parar Container**
```bash
# Linux/macOS
make docker-stop
./pipeline.sh docker-stop

# Windows
.\pipeline.ps1 docker-stop
```

## 🔧 **Ferramentas de Desenvolvimento**

### **Instalar Ferramentas**
```bash
# Linux/macOS
make install-tools
./pipeline.sh install-tools

# Windows
.\pipeline.ps1 install-tools
```

### **Ferramentas Instaladas**
- `cargo-watch` - Executa comandos em modo watch
- `cargo-tarpaulin` - Cobertura de testes
- `cargo-audit` - Verificação de segurança
- `cargo-deny` - Análise de dependências

## 📊 **Status do Projeto**

### **Verificar Status**
```bash
# Linux/macOS
make status
./pipeline.sh status

# Windows
.\pipeline.ps1 status
```

### **Informações Exibidas**
- Versão do Rust
- Versão do Cargo
- Sistema operacional
- Status das ferramentas

## 🚀 **GitHub Actions Pipeline**

### **Jobs Executados**

#### **1. Build and Test**
- **Sistema**: Ubuntu Latest
- **Serviços**: SQL Server + Redis
- **Etapas**:
  - Checkout do código
  - Instalação Rust toolchain
  - Cache de dependências
  - Setup do banco de dados
  - Verificação de formatação
  - Execução do clippy
  - Execução dos testes
  - Build release
  - Upload de artifacts

#### **2. Security Analysis**
- **Sistema**: Ubuntu Latest
- **Etapas**:
  - Checkout do código
  - Instalação Rust toolchain
  - Instalação cargo-audit
  - Execução de auditoria de segurança

#### **3. Code Quality**
- **Sistema**: Ubuntu Latest
- **Etapas**:
  - Checkout do código
  - Instalação Rust toolchain
  - Verificação de formatação
  - Execução do clippy
  - Análise de dependências

#### **4. Deploy to Production**
- **Sistema**: Ubuntu Latest
- **Trigger**: Apenas na branch `main`
- **Dependências**: Todos os jobs anteriores
- **Etapas**:
  - Download de artifacts
  - Preparação para deploy

### **Configuração do Banco de Dados**
```yaml
services:
  sqlserver:
    image: mcr.microsoft.com/mssql/server:2019-latest
    env:
      ACCEPT_EULA: Y
      SA_PASSWORD: YourStrong@Passw0rd
      MSSQL_PID: Developer
    ports:
      - 1433:1433
```

### **Configuração do Redis**
```yaml
services:
  redis:
    image: redis:7-alpine
    ports:
      - 6379:6379
```

## 🔍 **Verificações de Qualidade**

### **1. Formatação (cargo fmt)**
- Verifica se o código está formatado corretamente
- Aplica formatação automática se necessário

### **2. Linting (cargo clippy)**
- Identifica problemas de código
- Sugere melhorias
- Aplica regras de estilo Rust

### **3. Testes (cargo test)**
- Executa todos os testes unitários
- Executa testes de integração
- Verifica cobertura de código

### **4. Segurança (cargo audit)**
- Verifica vulnerabilidades conhecidas
- Analisa dependências
- Reporta problemas de segurança

## 📈 **Métricas e Relatórios**

### **Cobertura de Testes**
```bash
# Linux/macOS
make test-coverage
./pipeline.sh test-coverage

# Windows
# Instalar cargo-tarpaulin primeiro
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### **Análise de Dependências**
```bash
# Linux/macOS
make deps-tree
./pipeline.sh deps-tree

# Windows
cargo tree
```

## 🛠️ **Solução de Problemas**

### **Erro: "cargo-audit não encontrado"**
```bash
# Instalar ferramentas
make install-tools
# ou
cargo install cargo-audit
```

### **Erro: "cargo-watch não encontrado"**
```bash
# Instalar ferramentas
make install-tools
# ou
cargo install cargo-watch
```

### **Erro: "Docker não encontrado"**
- Instalar Docker Desktop (Windows/macOS)
- Instalar Docker Engine (Linux)
- Verificar se o Docker está rodando

### **Erro: "SQL Server não conecta"**
- Verificar se o SQL Server está rodando
- Verificar credenciais no `.env`
- Verificar firewall e portas

## 🎉 **Benefícios dos Pipelines**

### **1. Automatização**
- Execução automática de verificações
- Redução de erros humanos
- Consistência no processo de build

### **2. Qualidade**
- Verificação automática de formatação
- Linting automático
- Testes automáticos

### **3. Segurança**
- Verificação automática de vulnerabilidades
- Análise de dependências
- Auditoria de segurança

### **4. Produtividade**
- Comandos simples e intuitivos
- Feedback rápido sobre problemas
- Integração com CI/CD

### **5. Portabilidade**
- Funciona em Windows, Linux e macOS
- Scripts específicos para cada plataforma
- Comandos consistentes entre plataformas

## 📚 **Recursos Adicionais**

- **Rust Book**: https://doc.rust-lang.org/book/
- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **GitHub Actions**: https://docs.github.com/en/actions
- **Docker**: https://docs.docker.com/
- **SQL Server**: https://docs.microsoft.com/en-us/sql/

---

**🎯 Use os pipelines para manter a qualidade e produtividade do seu projeto Rust!**
