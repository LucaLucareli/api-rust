#!/bin/bash

# Este script é projetado para ser executado EXCLUSIVAMENTE DENTRO do contêiner Docker do SQL Server.
# Ele espera que as variáveis de ambiente e os arquivos de migração estejam presentes internamente.

echo "Iniciando processo de migração dentro do contêiner..."

# Caminho para o arquivo .env DENTRO do contêiner (onde foi copiado pelo script externo)
ENV_FILE="/tmp/migration_setup/.env"

# Verifica se o arquivo .env existe dentro do contêiner
if [ ! -f "$ENV_FILE" ]; then
    echo "❌ Erro: Arquivo .env não encontrado dentro do contêiner em '$ENV_FILE'."
    exit 1
fi

# Exporta as variáveis do .env para o ambiente do script dentro do contêiner
echo "Carregando variáveis de ambiente de '$ENV_FILE'..."
export $(grep -v '^#' "$ENV_FILE" | xargs)

# --- Configurações de Conexão SQL Server (A partir de DENTRO do Contêiner) ---
# Quando o script roda DENTRO do contêiner SQL Server, ele se conecta a si mesmo.
# Portanto, o host é 'localhost' ou '127.0.0.1' e a porta é a porta INTERNA padrão do SQL Server (1433).
SERVER="localhost,1433"
USER="$DB_USER"
PASSWORD="$DB_PASSWORD"
DATABASE="$DB_NAME"

# Verificação de variáveis de ambiente essenciais carregadas
if [ -z "$USER" ] || [ -z "$PASSWORD" ] || [ -z "$DATABASE" ]; then
    echo "❌ Erro: Variáveis de ambiente DB_USER, DB_PASSWORD ou DB_NAME não definidas APÓS carregar o .env dentro do contêiner."
    echo "Verifique o conteúdo do seu .env."
    exit 1
fi


# Pasta onde os arquivos .sql foram copiados DENTRO do contêiner
MIGRATIONS_DIR="/tmp/migration_setup/migrations"

# Verifica se o diretório de migrações existe dentro do contêiner
if [ ! -d "$MIGRATIONS_DIR" ]; then
    echo "❌ Erro: Diretório de migrações '$MIGRATIONS_DIR' não encontrado dentro do contêiner."
    exit 1
fi

# Verifica se o sqlcmd está instalado dentro do contêiner
if ! command -v sqlcmd &> /dev/null; then
    echo "❌ Erro: 'sqlcmd' não está instalado ou não está no PATH DENTRO do contêiner."
    echo "Certifique-se de que a imagem Docker 'api_rust-sqlserver' inclui o sqlcmd (geralmente 'mssql-tools')."
    exit 1
fi

echo "Conectando ao SQL Server em '$SERVER', Banco: '$DATABASE', Usuário: '$USER'..."

# Executa os arquivos de migração ordenadamente
for FILE in $(ls "$MIGRATIONS_DIR"/*.sql | sort); do
    echo "-> Executando migração: $FILE"
    # Conecta ao SQL Server usando as credenciais e o banco de dados
    sqlcmd -S "$SERVER" -U "$USER" -P "$PASSWORD" -d "$DATABASE" -i "$FILE"

    if [ $? -ne 0 ]; then
        echo "❌ Erro ao executar $FILE no banco de dados. Verifique a saída acima para detalhes."
        exit 1
    else
        echo "✅ Migração $FILE aplicada com sucesso."
    fi
done

echo "🎉 Todas as migrações foram aplicadas com sucesso dentro do contêiner!"