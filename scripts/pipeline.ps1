# Pipeline PowerShell para API Rust Monorepo
# Uso: .\pipeline.ps1 [comando]

param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

# Cores para output
$Green = "`e[32m"
$Yellow = "`e[33m"
$Red = "`e[31m"
$Reset = "`e[0m"

# Função para mostrar mensagens coloridas
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    
    switch ($Color) {
        "Green" { Write-Host $Message -ForegroundColor Green }
        "Yellow" { Write-Host $Message -ForegroundColor Yellow }
        "Red" { Write-Host $Message -ForegroundColor Red }
        default { Write-Host $Message }
    }
}

# Função para mostrar ajuda
function Show-Help {
    Write-ColorOutput "🚀 API Rust Monorepo - Comandos disponíveis:" "Green"
    Write-ColorOutput ""
    Write-ColorOutput "  build          - Compila o projeto em modo debug" "Yellow"
    Write-ColorOutput "  build-release  - Compila o projeto em modo release" "Yellow"
    Write-ColorOutput "  test           - Executa todos os testes" "Yellow"
    Write-ColorOutput "  clean          - Remove arquivos de build" "Yellow"
    Write-ColorOutput "  fmt            - Formata o código" "Yellow"
    Write-ColorOutput "  fmt-check      - Verifica formatação do código" "Yellow"
    Write-ColorOutput "  clippy         - Executa clippy (linter)" "Yellow"
    Write-ColorOutput "  check          - Executa todas as verificações" "Yellow"
    Write-ColorOutput "  lint           - Formata e executa clippy" "Yellow"
    Write-ColorOutput "  security       - Verifica vulnerabilidades" "Yellow"
    Write-ColorOutput "  docker-build   - Constrói imagem Docker" "Yellow"
    Write-ColorOutput "  docker-run     - Executa container Docker" "Yellow"
    Write-ColorOutput "  docker-stop    - Para container Docker" "Yellow"
    Write-ColorOutput "  dev            - Inicia modo desenvolvimento" "Yellow"
    Write-ColorOutput "  pipeline       - Executa pipeline completo" "Yellow"
    Write-ColorOutput "  validate       - Valida código para produção" "Yellow"
    Write-ColorOutput "  status         - Mostra status do projeto" "Yellow"
    Write-ColorOutput "  install-tools  - Instala ferramentas" "Yellow"
    Write-ColorOutput "  help           - Mostra esta ajuda" "Yellow"
    Write-ColorOutput ""
}

# Função para verificar se o Rust está instalado
function Test-RustInstallation {
    try {
        $rustVersion = rustc --version
        $cargoVersion = cargo --version
        Write-ColorOutput "✅ Rust instalado: $rustVersion" "Green"
        Write-ColorOutput "✅ Cargo instalado: $cargoVersion" "Green"
        return $true
    }
    catch {
        Write-ColorOutput "❌ Rust não está instalado ou não está no PATH" "Red"
        Write-ColorOutput "Instale o Rust em: https://rustup.rs/" "Yellow"
        return $false
    }
}

# Função para build
function Build-Project {
    Write-ColorOutput "🔨 Compilando projeto..." "Green"
    cargo build
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Compilação concluída!" "Green"
    } else {
        Write-ColorOutput "❌ Erro na compilação" "Red"
        exit 1
    }
}

# Função para build release
function Build-ProjectRelease {
    Write-ColorOutput "🚀 Compilando projeto em modo release..." "Green"
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Compilação release concluída!" "Green"
    } else {
        Write-ColorOutput "❌ Erro na compilação release" "Red"
        exit 1
    }
}

# Função para testes
function Test-Project {
    Write-ColorOutput "🧪 Executando testes..." "Green"
    cargo test
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Testes concluídos!" "Green"
    } else {
        Write-ColorOutput "❌ Erro nos testes" "Red"
        exit 1
    }
}

# Função para limpeza
function Clean-Project {
    Write-ColorOutput "🧹 Limpando arquivos de build..." "Yellow"
    cargo clean
    Write-ColorOutput "✅ Limpeza concluída!" "Green"
}

# Função para formatação
function Format-Code {
    Write-ColorOutput "🎨 Formatando código..." "Green"
    cargo fmt --all
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Formatação concluída!" "Green"
    } else {
        Write-ColorOutput "❌ Erro na formatação" "Red"
        exit 1
    }
}

# Função para verificar formatação
function Test-Format {
    Write-ColorOutput "🔍 Verificando formatação..." "Green"
    cargo fmt --all -- --check
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Formatação OK!" "Green"
    } else {
        Write-ColorOutput "❌ Formatação incorreta" "Red"
        exit 1
    }
}

# Função para clippy
function Invoke-Clippy {
    Write-ColorOutput "🔍 Executando clippy..." "Green"
    cargo clippy --all-targets --all-features -- -D warnings
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Clippy OK!" "Green"
    } else {
        Write-ColorOutput "❌ Clippy encontrou problemas" "Red"
        exit 1
    }
}

# Função para verificação de segurança
function Test-Security {
    Write-ColorOutput "🔒 Verificando segurança..." "Green"
    try {
        cargo audit
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✅ Verificação de segurança concluída!" "Green"
        } else {
            Write-ColorOutput "⚠️ Vulnerabilidades encontradas" "Yellow"
        }
    }
    catch {
        Write-ColorOutput "⚠️ cargo-audit não está instalado" "Yellow"
        Write-ColorOutput "Instale com: cargo install cargo-audit" "Yellow"
    }
}

# Função para Docker
function Build-Docker {
    Write-ColorOutput "🐳 Construindo imagem Docker..." "Green"
    docker build -t api-rust:latest .
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Imagem Docker construída!" "Green"
    } else {
        Write-ColorOutput "❌ Erro na construção Docker" "Red"
        exit 1
    }
}

function Start-Docker {
    Write-ColorOutput "🚀 Iniciando container Docker..." "Green"
    docker-compose up -d
    Write-ColorOutput "✅ Container iniciado!" "Green"
}

function Stop-Docker {
    Write-ColorOutput "⏹️ Parando container Docker..." "Yellow"
    docker-compose down
    Write-ColorOutput "✅ Container parado!" "Green"
}

# Função para desenvolvimento
function Start-Development {
    Write-ColorOutput "🔄 Iniciando modo desenvolvimento..." "Green"
    cargo run
}

# Função para pipeline completo
function Invoke-Pipeline {
    Write-ColorOutput "🚀 Executando pipeline completo..." "Green"
    Clean-Project
    Test-Format
    Invoke-Clippy
    Test-Project
    Build-ProjectRelease
    Write-ColorOutput "🎉 Pipeline completo executado com sucesso!" "Green"
}

# Função para validação
function Test-Validation {
    Write-ColorOutput "🎯 Validando código para produção..." "Green"
    Test-Format
    Invoke-Clippy
    Test-Security
    Test-Project
    Build-ProjectRelease
    Write-ColorOutput "🎯 Código validado para produção!" "Green"
}

# Função para status
function Show-Status {
    Write-ColorOutput "📊 Status do Projeto:" "Green"
    try {
        $rustVersion = rustc --version
        $cargoVersion = cargo --version
        Write-ColorOutput "Rust version: $rustVersion" "White"
        Write-ColorOutput "Cargo version: $cargoVersion" "White"
        
        # Verificar se é Windows
        if ($IsWindows -or $env:OS -eq "Windows_NT") {
            Write-ColorOutput "Sistema: Windows" "White"
        } else {
            Write-ColorOutput "Sistema: Unix/Linux/macOS" "White"
        }
        
        Write-ColorOutput "✅ Status verificado!" "Green"
    }
    catch {
        Write-ColorOutput "❌ Erro ao verificar status" "Red"
    }
}

# Função para instalar ferramentas
function Install-Tools {
    Write-ColorOutput "📦 Instalando ferramentas..." "Green"
    try {
        cargo install cargo-watch
        cargo install cargo-tarpaulin
        cargo install cargo-audit
        cargo install cargo-deny
        Write-ColorOutput "✅ Ferramentas instaladas!" "Green"
    }
    catch {
        Write-ColorOutput "❌ Erro ao instalar ferramentas" "Red"
    }
}

# Main execution
Write-ColorOutput "🚀 API Rust Monorepo - Pipeline PowerShell" "Green"
Write-ColorOutput "=============================================" "Green"

# Verificar instalação do Rust
if (-not (Test-RustInstallation)) {
    exit 1
}

# Executar comando solicitado
switch ($Command.ToLower()) {
    "build" { Build-Project }
    "build-release" { Build-ProjectRelease }
    "test" { Test-Project }
    "clean" { Clean-Project }
    "fmt" { Format-Code }
    "fmt-check" { Test-Format }
    "clippy" { Invoke-Clippy }
    "check" { 
        Test-Format
        Invoke-Clippy
        Write-ColorOutput "✅ Todas as verificações passaram!" "Green"
    }
    "lint" { 
        Format-Code
        Invoke-Clippy
        Write-ColorOutput "✅ Linting concluído!" "Green"
    }
    "security" { Test-Security }
    "docker-build" { Build-Docker }
    "docker-run" { Start-Docker }
    "docker-stop" { Stop-Docker }
    "dev" { Start-Development }
    "pipeline" { Invoke-Pipeline }
    "validate" { Test-Validation }
    "status" { Show-Status }
    "install-tools" { Install-Tools }
    "help" { Show-Help }
    default { 
        Write-ColorOutput "❌ Comando desconhecido: $Command" "Red"
        Show-Help
        exit 1
    }
}
