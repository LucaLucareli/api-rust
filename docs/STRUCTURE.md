# 🏗️ Estrutura do Projeto - API Rust Monorepo

Este documento descreve a estrutura organizada do projeto após a reorganização.

## 📁 **Estrutura de Diretórios**

```
api-rust/
├── .github/                    # GitHub Actions (CI/CD)
│   └── workflows/
│       └── rust.yml           # Pipeline de CI/CD
├── .vscode/                    # Configurações do VS Code
├── config/                     # Configurações do projeto
│   ├── config.rs              # Configurações Rust
│   ├── project.toml           # Configuração do projeto
│   ├── schema.sql             # Schema do banco de dados
│   └── migrations/            # Migrações do banco
│       ├── migrate.sh         # Script de migração
│       └── [timestamp]-[name]/ # Pastas de migração
├── docs/                       # Documentação
│   ├── README.md              # Documentação principal
│   ├── STRUCTURE.md           # Este arquivo
│   └── PIPELINE_README.md     # Documentação dos pipelines
├── scripts/                    # Scripts de automação
│   ├── pipeline.sh            # Pipeline Bash (Linux/macOS)
│   ├── pipeline.ps1           # Pipeline PowerShell (Windows)
│   ├── Makefile               # Makefile (Linux/macOS)
│   ├── generate_migration.sh  # Gerador de migrações
│   ├── run_migrations_on_docker.sh # Executor de migrações Docker
│   └── run_migrations_internal.sh  # Executor de migrações internas
├── src/                        # Código fonte principal
│   ├── main.rs                # Ponto de entrada da aplicação
│   ├── lib.rs                 # Biblioteca principal
│   ├── errors.rs              # Tratamento de erros
│   ├── apps/                  # APIs individuais
│   │   ├── auth_api/          # API de Autenticação
│   │   ├── admin_api/         # API de Administração
│   │   └── viewer_api/        # API de Visualização
│   ├── libs/                  # Bibliotecas compartilhadas
│   │   └── shared/            # Módulos compartilhados
│   │       ├── cache/         # Sistema de cache
│   │       ├── database/      # Camada de dados
│   │       │   └── repositories/ # Repositórios
│   │       ├── logging/       # Sistema de logging
│   │       └── modules/       # Módulos específicos
│   │           ├── auth/      # Módulo de autenticação
│   │           └── jwt/       # Módulo JWT
│   └── infrastructure/        # Infraestrutura
│       ├── db.rs              # Conexão com banco
│       └── redis.rs           # Conexão com Redis
├── docker/                     # Configurações Docker
│   ├── sqlserver/             # Configuração SQL Server
│   │   ├── Dockerfile         # Imagem SQL Server
│   │   ├── entrypoint.sh      # Script de inicialização
│   │   ├── setup.sql          # Setup inicial
│   │   └── configure-db.sh    # Configuração do banco
│   └── redis/                 # Configuração Redis
│       └── Dockerfile         # Imagem Redis
├── .gitignore                  # Arquivos ignorados pelo Git
├── Cargo.toml                  # Dependências Rust
├── Cargo.lock                  # Lock das dependências
├── README.md                   # Documentação principal
├── docker-compose.yml          # Orquestração Docker
└── Dockerfile                  # Imagem da aplicação
```

## 🔄 **Organização por Funcionalidade**

### **1. Configurações (`/config`)**
- **config.rs**: Configurações da aplicação Rust
- **project.toml**: Configuração centralizada do projeto
- **schema.sql**: Schema do banco de dados
- **migrations/**: Sistema de migrações

### **2. Scripts (`/scripts`)**
- **pipeline.sh**: Pipeline Bash para Linux/macOS
- **pipeline.ps1**: Pipeline PowerShell para Windows
- **Makefile**: Comandos make para Linux/macOS
- **generate_migration.sh**: Gerador de migrações
- **run_migrations_*.sh**: Executores de migrações

### **3. Documentação (`/docs`)**
- **README.md**: Documentação principal
- **STRUCTURE.md**: Este arquivo de estrutura
- **PIPELINE_README.md**: Documentação dos pipelines

### **4. Código Fonte (`/src`)**
- **main.rs**: Ponto de entrada da aplicação
- **lib.rs**: Biblioteca principal
- **errors.rs**: Tratamento de erros
- **apps/**: APIs individuais (Auth, Admin, Viewer)
- **libs/shared/**: Módulos compartilhados
- **infrastructure/**: Infraestrutura (DB, Redis)

### **5. Docker (`/docker`)**
- **sqlserver/**: Configuração do SQL Server
- **redis/**: Configuração do Redis

## 🎯 **Princípios de Organização**

### **1. Separação de Responsabilidades**
- **Configurações**: Centralizadas em `/config`
- **Scripts**: Agrupados em `/scripts`
- **Documentação**: Organizada em `/docs`
- **Código**: Estruturado em `/src`

### **2. Hierarquia Clara**
- Cada tipo de arquivo tem sua pasta específica
- Subpastas organizadas por funcionalidade
- Nomenclatura consistente e descritiva

### **3. Facilidade de Manutenção**
- Scripts relacionados agrupados
- Configurações centralizadas
- Documentação organizada por tópico

### **4. Portabilidade**
- Scripts para diferentes sistemas operacionais
- Configurações independentes de plataforma
- Estrutura que funciona em Windows, Linux e macOS

## 🚀 **Benefícios da Nova Estrutura**

### **1. Organização**
- Arquivos relacionados agrupados
- Fácil localização de recursos
- Estrutura intuitiva para novos desenvolvedores

### **2. Manutenibilidade**
- Separação clara de responsabilidades
- Fácil atualização de configurações
- Scripts organizados por função

### **3. Escalabilidade**
- Estrutura preparada para crescimento
- Fácil adição de novas APIs
- Organização que suporta equipes grandes

### **4. Padrões**
- Segue convenções do Rust
- Estrutura padrão para monorepos
- Organização profissional e madura

## 📋 **Arquivos Removidos/Consolidados**

### **Scripts Removidos**
- `build.sh` → Substituído pelos pipelines
- `dev.sh` → Substituído pelos pipelines
- `test_api.sh` → Funcionalidade integrada nos pipelines

### **Scripts Mantidos**
- `pipeline.sh` → Pipeline principal para Linux/macOS
- `pipeline.ps1` → Pipeline principal para Windows
- `Makefile` → Comandos make para Linux/macOS
- `generate_migration.sh` → Gerador de migrações
- `run_migrations_*.sh` → Executores de migrações

## 🔧 **Como Usar a Nova Estrutura**

### **Desenvolvimento**
```bash
# Usar pipelines (recomendado)
./scripts/pipeline.sh dev
./scripts/pipeline.ps1 dev

# Usar make (Linux/macOS)
make dev

# Executar migrações
./scripts/run_migrations_on_docker.sh
```

### **Build e Testes**
```bash
# Pipeline completo
./scripts/pipeline.sh pipeline
./scripts/pipeline.ps1 pipeline

# Validação para produção
./scripts/pipeline.sh validate
./scripts/pipeline.ps1 validate
```

### **Configurações**
- Editar `config/config.rs` para configurações Rust
- Editar `config/project.toml` para configurações do projeto
- Editar `config/schema.sql` para mudanças no banco

## 📚 **Próximos Passos**

1. **Testar a nova estrutura** com os pipelines
2. **Atualizar documentação** conforme necessário
3. **Adicionar novas funcionalidades** seguindo a estrutura
4. **Manter consistência** na organização

---

**🎯 A nova estrutura torna o projeto mais profissional, organizado e fácil de manter!**
