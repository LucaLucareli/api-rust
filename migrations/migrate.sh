#!/bin/bash

# Script de migração para o sistema de streaming
# Similar ao Prisma, mas usando arquivos SQL

set -e

MIGRATIONS_DIR="migrations"
SCHEMA_FILE="schema.sql"
TIMESTAMP=$(date +"%Y%m%d%H%M%S")

echo "🚀 Sistema de Migrações - API Rust Streaming"
echo "=============================================="

# Função para criar nova migração
create_migration() {
    local name=$1
    local migration_dir="${MIGRATIONS_DIR}/${TIMESTAMP}_${name}"
    
    echo "📝 Criando nova migração: ${migration_dir}"
    
    mkdir -p "${migration_dir}"
    
    # Criar arquivo de migração
    cat > "${migration_dir}/migration.sql" << EOF
-- Migração: ${name}
-- Timestamp: ${TIMESTAMP}
-- Descrição: 

-- Adicione suas alterações SQL aqui
-- Exemplo:
-- CREATE TABLE nova_tabela (
--     id VARCHAR(36) PRIMARY KEY,
--     nome VARCHAR(255) NOT NULL,
--     created_at DATETIME2 DEFAULT GETDATE()
-- );

-- Para alterar tabelas existentes:
-- ALTER TABLE tabela_existente ADD nova_coluna VARCHAR(255);

-- Para criar índices:
-- CREATE INDEX idx_nome ON tabela(coluna);

-- Para inserir dados:
-- INSERT INTO tabela (coluna1, coluna2) VALUES ('valor1', 'valor2');

EOF

    # Criar arquivo README
    cat > "${migration_dir}/README.md" << EOF
# Migração: ${name}

## Descrição
Descreva o que esta migração faz.

## Alterações
- [ ] Adicionar tabela
- [ ] Alterar coluna
- [ ] Criar índice
- [ ] Inserir dados

## SQL Executado
\`\`\`sql
-- Cole aqui o SQL que foi executado
\`\`\`

## Rollback
\`\`\`sql
-- SQL para reverter as alterações se necessário
\`\`\`
EOF

    echo "✅ Migração criada em: ${migration_dir}"
    echo "📁 Edite o arquivo migration.sql e execute: ./migrations/run_migrations_on_docker.sh"
}

# Função para listar migrações
list_migrations() {
    echo "📋 Migrações disponíveis:"
    echo ""
    
    if [ ! -d "$MIGRATIONS_DIR" ]; then
        echo "❌ Diretório de migrações não encontrado"
        return
    fi
    
    for migration in $(ls -1 "$MIGRATIONS_DIR" | sort); do
        if [ -d "$MIGRATIONS_DIR/$migration" ]; then
            echo "  📁 $migration"
            if [ -f "$MIGRATIONS_DIR/$migration/README.md" ]; then
                echo "     $(head -n 1 "$MIGRATIONS_DIR/$migration/README.md" | sed 's/# //')"
            fi
        fi
    done
}

# Função para mostrar status das migrações
status_migrations() {
    echo "📊 Status das Migrações"
    echo "======================="
    
    # TODO: Implementar verificação no banco de dados
    echo "🔍 Verificando status no banco..."
    echo "⚠️  Funcionalidade em desenvolvimento"
}

# Função para mostrar ajuda
show_help() {
    echo "Uso: $0 [COMANDO] [NOME_MIGRACAO]"
    echo ""
    echo "Comandos:"
    echo "  create <nome>    - Criar nova migração"
    echo "  list             - Listar migrações disponíveis"
    echo "  status           - Mostrar status das migrações"
    echo "  help             - Mostrar esta ajuda"
    echo ""
    echo "Exemplos:"
    echo "  $0 create add_users_table"
    echo "  $0 create alter_videos_rating"
    echo "  $0 list"
    echo "  $0 status"
}

# Verificar argumentos
case "${1:-help}" in
    "create")
        if [ -z "$2" ]; then
            echo "❌ Erro: Nome da migração é obrigatório"
            echo "Uso: $0 create <nome_migracao>"
            exit 1
        fi
        create_migration "$2"
        ;;
    "list")
        list_migrations
        ;;
    "status")
        status_migrations
        ;;
    "help"|*)
        show_help
        ;;
esac
