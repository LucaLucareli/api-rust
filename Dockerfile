# Usar a imagem oficial do Rust
FROM rust:1.75-slim as builder

# Definir diretório de trabalho
WORKDIR /usr/src/app

# Copiar arquivos de dependências
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Construir a aplicação
RUN cargo build --release

# Imagem de produção
FROM debian:bookworm-slim

# Instalar dependências necessárias
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN useradd -m -u 1000 rustuser

# Copiar binário da aplicação
COPY --from=builder /usr/src/app/target/release/api_rust /usr/local/bin/

# Definir diretório de trabalho
WORKDIR /app

# Copiar arquivo de configuração
COPY env.example .env

# Mudar para usuário não-root
USER rustuser

# Expor porta
EXPOSE 8000

# Comando para executar a aplicação
CMD ["api_rust"]
