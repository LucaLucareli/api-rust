# API Rust Monorepo - Sistema de Streaming com JWT, Cache e Banco Real

Este é um projeto monorepo em Rust que implementa um sistema de streaming de vídeos similar ao Netflix, com múltiplas APIs especializadas, autenticação JWT, cache Redis, banco de dados real e sistema de migrações:

- **Auth API** (`/auth`) - Autenticação JWT com refresh tokens e registro público
- **Admin API** (`/admin`) - Gerenciamento de conteúdo (requer autenticação admin)
- **Viewer API** (`/viewer`) - Visualização de catálogo com cache

## 🏗️ Estrutura do Projeto

```
api_rust/
├── Cargo.toml                 # Workspace principal
├── schema.sql                 # Schema do banco de dados
├── migrations/                # Sistema de migrações
│   ├── migrate.sh            # Script para criar migrações
│   └── [timestamp]_[nome]/   # Pastas de migração
├── src/
│   ├── main.rs               # Ponto de entrada principal (API Gateway)
│   ├── lib.rs                # Biblioteca compartilhada
│   ├── config.rs             # Configurações centralizadas
│   ├── errors.rs             # Tratamento de erros padronizado
│   └── infrastructure/       # Infraestrutura compartilhada
│       ├── db.rs             # Conexão com banco de dados
│       └── redis.rs          # Cliente Redis
├── src/libs/shared/          # Funcionalidades compartilhadas
│   ├── modules/              # Módulos compartilhados
│   │   ├── jwt.rs            # Gerenciamento JWT
│   │   └── auth.rs           # Middleware de autenticação
│   ├── database/             # Camada de dados
│   │   ├── repositories/     # Repositórios para cada tabela
│   │   │   ├── users.rs      # CRUD de usuários com SQL real
│   │   │   ├── videos.rs     # CRUD de vídeos com SQL real
│   │   │   └── ...           # Outros repositórios
│   │   └── connection.rs     # Conexão com banco
│   ├── cache/                # Sistema de cache
│   │   └── redis_cache.rs    # Cache Redis
│   └── logging/              # Logging automático
│       └── mod.rs            # Middleware de logs
├── src/apps/                 # APIs individuais
│   ├── auth_api/             # API de autenticação
│   │   ├── src/
│   │   │   ├── main.rs       # Ponto de entrada
│   │   │   ├── lib.rs        # Módulos da API
│   │   │   ├── dto/          # Data Transfer Objects
│   │   │   ├── controllers/  # Controllers REST
│   │   │   ├── services/     # Lógica de negócio
│   │   │   ├── models/       # Modelos de dados
│   │   │   └── routes.rs     # Definição de rotas
│   │   └── Cargo.toml
│   ├── admin_api/            # API administrativa
│   │   ├── src/
│   │   │   ├── main.rs       # Ponto de entrada
│   │   │   ├── lib.rs        # Módulos da API
│   │   │   ├── dto/          # Data Transfer Objects
│   │   │   ├── controllers/  # Controllers REST
│   │   │   ├── services/     # Lógica de negócio
│   │   │   ├── models/       # Modelos de dados
│   │   │   └── routes.rs     # Definição de rotas
│   │   └── Cargo.toml
│   └── viewer_api/           # API de visualização
│       ├── src/
│       │   ├── main.rs       # Ponto de entrada
│       │   ├── lib.rs        # Módulos da API
│       │   ├── dto/          # Data Transfer Objects
│       │   ├── controllers/  # Controllers REST
│       │   ├── services/     # Lógica de negócio
│       │   ├── models/       # Modelos de dados
│       │   └── routes.rs     # Definição de rotas
│       └── Cargo.toml
├── docker/                   # Configurações Docker
├── docker-compose.yml        # Orquestração dos serviços
├── run_migrations_on_docker.sh  # Script de migrações
├── run_migrations_internal.sh   # Script interno de migrações
├── build.sh                  # Script de build
├── dev.sh                    # Script de desenvolvimento
└── env.example               # Exemplo de variáveis de ambiente
```

## 🚀 Funcionalidades Implementadas

### ✅ **Autenticação JWT Completa**
- **Access Token**: Validade de 1 hora (configurável via .env)
- **Refresh Token**: Validade de 7 dias (configurável via .env)
- **Middleware de autenticação**: Protege rotas privadas
- **Middleware de admin**: Verifica permissões de administrador
- **Rota de registro pública**: `/auth/register` (sem autenticação)

### ✅ **Sistema de Banco de Dados Real**
- **SQL Server**: Banco de dados principal
- **Consultas SQL reais**: Sem mocks ou simulações
- **Repositórios funcionais**: CRUD completo implementado
- **Hash de senhas**: Bcrypt para segurança
- **Relacionamentos**: Foreign keys e constraints

### ✅ **Sistema de Cache**
- **Cache Redis**: Implementado com TTL configurável
- **Cache automático**: Para endpoints de leitura (ex: catálogo de vídeos)
- **Funções de cache**: SET, GET, DELETE, EXISTS, INCREMENT

### ✅ **Repositórios de Banco de Dados**
- **Padrão Repository**: Uma pasta para cada tabela
- **Operações CRUD**: Create, Read, Update, Delete
- **Operações customizadas**: Busca por filtros, contagem, etc.
- **SQL parametrizado**: Proteção contra SQL injection
- **Estrutura organizada**: Fácil manutenção e extensão

### ✅ **Sistema de Migrações**
- **Schema SQL**: Arquivo `schema.sql` com estrutura completa
- **Script de migração**: `./migrations/migrate.sh create <nome>`
- **Timestamps automáticos**: Nomenclatura organizada
- **Templates**: Arquivos README e SQL para cada migração
- **Similar ao Prisma**: Fácil de usar e manter

### ✅ **DTOs Organizados**
- **Pasta dedicada**: `dto/` em cada API
- **Separação clara**: Request, Response e estruturas internas
- **Validação**: Serde para serialização/deserialização
- **Enums para roles**: Admin e Viewer com conversões

### ✅ **Logging Automático**
- **Logs de requisições**: Método, URI, status, latência, User-Agent
- **Logs de negócio**: Autenticação, cache, banco de dados
- **Níveis configuráveis**: INFO, WARN, ERROR
- **Formato estruturado**: Fácil de ler e processar

## 🔐 Autenticação JWT

### Endpoints de Autenticação

#### `POST /auth/register` (PÚBLICO)
```json
{
  "email": "novo@usuario.com",
  "password": "senha123",
  "name": "Novo Usuário"
}
```

**Resposta:**
```json
{
  "user": {
    "id": "uuid-gerado",
    "email": "novo@usuario.com",
    "name": "Novo Usuário",
    "role": "Viewer"
  },
  "message": "Usuário registrado com sucesso"
}
```

#### `POST /auth/login`
```json
{
  "email": "admin@streaming.com",
  "password": "password"
}
```

**Resposta:**
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 1703123456,
  "refresh_expires_in": 1703728256,
  "user": {
    "id": "admin-001",
    "email": "admin@streaming.com",
    "name": "Admin",
    "role": "Admin"
  }
}
```

#### `POST /auth/refresh`
```json
{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

**Resposta:**
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 1703123456
}
```

### Uso do Token

Para acessar rotas protegidas, inclua o header:
```
Authorization: Bearer <access_token>
```

## 🗄️ Sistema de Migrações

### Como Usar

1. **Criar nova migração:**
   ```bash
   ./migrations/migrate.sh create add_new_table
   ```

2. **Editar a migração:**
   ```bash
   # Editar o arquivo SQL gerado
   vim migrations/20241201120000_add_new_table/migration.sql
   ```

3. **Executar migrações:**
   ```bash
   ./run_migrations_on_docker.sh
   ```

### Estrutura das Migrações

Cada migração cria uma pasta com:
- `migration.sql` - SQL para executar
- `README.md` - Documentação da migração

### Exemplo de Migração

```sql
-- Migração: add_new_table
-- Timestamp: 20241201120000
-- Descrição: Adicionar tabela de comentários

CREATE TABLE comments (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    video_id VARCHAR(36) NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (video_id) REFERENCES videos(id)
);

CREATE INDEX idx_comments_user ON comments(user_id);
CREATE INDEX idx_comments_video ON comments(video_id);
```

## 💾 Sistema de Cache

### Exemplo de Uso no Viewer API

A rota `GET /viewer/videos` implementa cache automático:

1. **Verifica cache**: Busca por chave `videos:catalog`
2. **Cache HIT**: Retorna dados do cache
3. **Cache MISS**: Busca do banco e salva no cache
4. **TTL configurável**: Padrão de 5 minutos

### Funções de Cache Disponíveis

```rust
// Salvar no cache
cache.set("key", &value).await?;

// Buscar do cache
let value: Option<Type> = cache.get("key").await?;

// Salvar com TTL
cache.set_with_ttl("key", &value, Duration::from_secs(300)).await?;

// Verificar existência
let exists = cache.exists("key").await?;

// Deletar
cache.delete("key").await?;

// Incrementar contador
let count = cache.increment("counter", 1).await?;
```

## 🗄️ Repositórios de Banco de Dados

### Estrutura dos Repositórios

Cada repositório implementa o padrão CRUD completo com SQL real:

```rust
pub struct UsersRepository {
    db: DatabaseConnection,
}

impl UsersRepository {
    pub async fn create(&self, request: CreateUserRequest) -> Result<User, DbErr>;
    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, DbErr>;
    pub async fn find_all(&self, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<User>, DbErr>;
    pub async fn update(&self, id: &str, request: UpdateUserRequest) -> Result<Option<User>, DbErr>;
    pub async fn delete(&self, id: &str) -> Result<bool, DbErr>;
    pub async fn find_by_role(&self, role: &str) -> Result<Vec<User>, DbErr>;
    pub async fn count(&self) -> Result<u64, DbErr>;
    pub async fn authenticate(&self, request: &LoginRequest) -> Result<Option<User>, DbErr>;
}
```

### Repositórios Disponíveis

- **UsersRepository**: Gerenciamento de usuários com hash bcrypt
- **VideosRepository**: Catálogo de vídeos com busca e filtros
- **CategoriesRepository**: Categorias/genres
- **ActorsRepository**: Atores
- **DirectorsRepository**: Diretores
- **AccessGroupsRepository**: Grupos de acesso
- **WatchHistoryRepository**: Histórico de visualização
- **FavoritesRepository**: Vídeos favoritos
- **RatingsRepository**: Avaliações

### Exemplo de SQL Real

```rust
// Buscar usuário por email
let sql = r#"
    SELECT id, email, password_hash, name, role, profile_picture_url, 
           subscription_status, subscription_expires_at, created_at, updated_at
    FROM users 
    WHERE email = @P1
"#;

let stmt = Statement::from_sql_and_values(
    sea_orm::DatabaseBackend::SqlServer,
    sql,
    vec![email.into()],
);

let result = User::find_by_statement(stmt)
    .one(&self.db)
    .await?;
```

## 📊 Logging Automático

### Logs de Requisições HTTP

O sistema registra automaticamente:
- **Método HTTP**: GET, POST, PUT, DELETE
- **URI**: Caminho da requisição
- **Status**: Código de resposta HTTP
- **Latência**: Tempo de processamento
- **User-Agent**: Navegador/cliente

### Logs de Negócio

```rust
// Autenticação
log_authentication_success(&user_id);
log_authentication_failure(&email, &reason);

// Cache
log_cache_hit(&key);
log_cache_miss(&key);

// Banco de dados
log_database_connection_success();
log_database_connection_error(&error);

// Autorização
log_authorization_failure(&user_id, &resource);
```

## 🚀 Como Executar

### 1. Pré-requisitos

- Rust 1.70+
- Docker e Docker Compose
- SQL Server (via Docker)
- Redis (via Docker)
- Azurite (Azure Storage Emulator via Docker)

### 2. Configuração

1. Copie o arquivo de ambiente:
   ```bash
   cp env.example .env
   ```

2. Edite o arquivo `.env` com suas configurações:
   ```bash
   # Configurações do SQL Server
   DB_USER=sa
   DB_PASSWORD=YourStrong@Passw0rd
   DB_NAME=rust_cast_db
   DB_PORT=1433
   
   # Configurações do Redis
   REDIS_CACHE_PORT=27002
   
   # Configurações da API Principal
   API_PORT=8000
   RUST_LOG=info
   
   # Configurações das APIs individuais
   AUTH_API_PORT=3001
   ADMIN_API_PORT=3002
   VIEWER_API_PORT=3003
   
   # Configurações JWT
   JWT_ACCESS_SECRET=your-super-secret-access-key-here
   JWT_REFRESH_SECRET=your-super-secret-refresh-key-here
   JWT_ACCESS_EXPIRY_HOURS=1
   JWT_REFRESH_EXPIRY_DAYS=7
   
   # Configurações do Azurite
   AZURITE_ACCOUNT_NAME=devstoreaccount1
   AZURITE_ACCOUNT_KEY=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==
   AZURITE_BLOB_PORT=10000
   AZURITE_QUEUE_PORT=10001
   AZURITE_TABLE_PORT=10002
   ```

### 3. Iniciar Serviços

```bash
# Iniciar SQL Server, Redis e Azurite
./dev.sh docker

# Ou manualmente:
docker-compose up -d
```

### 4. Executar Migrações

```bash
# Executar migrações no banco de dados
./run_migrations_on_docker.sh
```

### 5. Executar as APIs

#### Opção 1: Executar todas as APIs individualmente
```bash
# Terminal 1 - Auth API
cargo run -p auth_api

# Terminal 2 - Admin API  
cargo run -p admin_api

# Terminal 3 - Viewer API
cargo run -p viewer_api

# Terminal 4 - API Principal
cargo run
```

#### Opção 2: Usar o script de desenvolvimento
```bash
# Executar API específica
./dev.sh run-auth      # Auth API na porta 3001
./dev.sh run-admin     # Admin API na porta 3002
./dev.sh run-viewer    # Viewer API na porta 3003
./dev.sh run           # API Principal na porta 8000
```

## 🔧 Desenvolvimento

### Arquitetura das APIs

Cada API segue o padrão **Controller-Service-Model-DTO**:

```
dto/           # Data Transfer Objects (Request/Response)
controllers/   # Handlers HTTP REST
services/      # Lógica de negócio
models/        # Estruturas de dados
routes.rs      # Definição de rotas
```

### Endpoints Disponíveis

#### Auth API (Porta 3001)
- `POST /auth/register` - Registro de usuário (PÚBLICO)
- `POST /auth/login` - Login com JWT
- `POST /auth/refresh` - Renovar access token
- `GET /auth/health` - Health check
- `GET /` - Status da API

#### Admin API (Porta 3002)
- `POST /admin/videos` - Criar novo vídeo (requer auth admin)
- `GET /admin/health` - Health check
- `GET /` - Status da API

#### Viewer API (Porta 3003)
- `GET /viewer/videos` - Listar catálogo com cache
- `GET /viewer/health` - Health check
- `GET /` - Status da API

#### API Principal (Porta 8000)
- `GET /` - Status do monorepo
- `GET /health` - Health check geral
- `GET /auth/*` - Proxy para Auth API
- `GET /admin/*` - Proxy para Admin API
- `GET /viewer/*` - Proxy para Viewer API

### Adicionar Nova API

1. Crie uma nova pasta em `src/apps/`
2. Adicione o `Cargo.toml` com dependências
3. Registre no `Cargo.toml` principal do workspace
4. Implemente a estrutura Controller-Service-Model-DTO
5. Adicione as rotas
6. Configure autenticação se necessário

## 🐳 Docker

### Serviços

- **SQL Server**: Banco de dados principal (Porta 1433)
- **Redis**: Cache e sessões (Porta 27002)
- **Azurite**: Azure Storage Emulator (Portas 10000-10002)

### Comandos Úteis

```bash
# Ver logs
docker-compose logs -f sqlserver
docker-compose logs -f redis
docker-compose logs -f azurite

# Parar serviços
docker-compose down

# Reconstruir
docker-compose build --no-cache

# Status dos serviços
docker-compose ps
```

## 📝 Scripts

### `dev.sh`
Script principal de desenvolvimento com comandos:
- `build` - Compila todas as APIs
- `run` - Executa a API principal
- `run-auth/admin/viewer` - Executa API específica
- `docker` - Inicia serviços Docker
- `migrate` - Executa migrações

### `migrations/migrate.sh`
Script para gerenciar migrações:
- `create <nome>` - Criar nova migração
- `list` - Listar migrações disponíveis
- `status` - Mostrar status das migrações
- `help` - Mostrar ajuda

### `run_migrations_on_docker.sh`
Script que orquestra migrações:
1. Verifica pré-requisitos
2. Aguarda contêiner ficar saudável
3. Copia arquivos para o contêiner
4. Executa migrações
5. Limpa arquivos temporários

### `run_migrations_internal.sh`
Script executado dentro do contêiner:
1. Carrega variáveis de ambiente
2. Conecta ao SQL Server
3. Executa arquivos de migração
4. Reporta status

## 🧪 Testes

```bash
# Testar todas as APIs
cargo test --workspace

# Testar API específica
cargo test -p auth_api
cargo test -p admin_api
cargo test -p viewer_api
```

## 📊 Monitoramento

- **Health Checks**: Cada API tem endpoint `/health`
- **Logs**: Configurados via `RUST_LOG` com tracing
- **Status**: Endpoint `/` em cada API mostra informações
- **Cache**: Logs automáticos de hit/miss
- **Autenticação**: Logs de sucesso/falha

## 🔒 Segurança

- **JWT Tokens**: Access e refresh tokens
- **Middleware de autenticação**: Protege rotas privadas
- **Middleware de admin**: Verifica permissões
- **Hash de senhas**: Bcrypt para segurança
- **SQL parametrizado**: Proteção contra SQL injection
- **Variáveis de ambiente**: Para credenciais e chaves
- **Validação de entrada**: Em todos os endpoints
- **Tratamento de erros**: Robusto e padronizado

## 🚀 Logs Estilo NestJS

O sistema exibe logs informativos ao iniciar:

```
🚀 Auth API iniciando em http://127.0.0.1:3001
📱 Endpoints disponíveis:
   - POST /auth/login
   - POST /auth/register
   - POST /auth/refresh
   - GET  /auth/health
   - GET  /
✅ Auth API pronta e rodando!
```

### Logs de Requisições

```
2024-01-01T10:00:00.000Z INFO Request: POST /auth/login - User-Agent: Mozilla/5.0...
2024-01-01T10:00:00.100Z INFO POST /auth/login 200 - 100ms - User-Agent: Mozilla/5.0...
```

### Logs de Cache

```
💾 Cache MISS - buscando do banco de dados
💾 Salvando resultado no cache
```

## 🤝 Contribuição

1. Fork o projeto
2. Crie uma branch para sua feature
3. Implemente seguindo o padrão Controller-Service-Model-DTO
4. Adicione autenticação se necessário
5. Implemente cache para endpoints de leitura
6. Adicione logs apropriados
7. Crie migrações para alterações no banco
8. Adicione testes
9. Commit suas mudanças
10. Push para a branch
11. Abra um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT.# api-rust
