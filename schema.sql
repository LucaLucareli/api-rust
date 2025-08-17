-- Schema do banco de dados para o sistema de streaming
-- Este arquivo define a estrutura das tabelas e relacionamentos
-- Para criar uma nova migração: ./migrations/migrate.sh create <nome>

-- =====================================================
-- TABELAS PRINCIPAIS
-- =====================================================

-- Tabela de usuários
CREATE TABLE users (
    id VARCHAR(36) PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'Viewer',
    profile_picture_url VARCHAR(500),
    subscription_status VARCHAR(50) DEFAULT 'active',
    subscription_expires_at DATETIME2,
    created_at DATETIME2 DEFAULT GETDATE(),
    updated_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de categorias
CREATE TABLE categories (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de vídeos
CREATE TABLE videos (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    duration_seconds INT NOT NULL,
    release_year INT,
    rating DECIMAL(3,2) DEFAULT 0.00,
    thumbnail_url VARCHAR(500),
    video_url VARCHAR(500),
    trailer_url VARCHAR(500),
    is_featured BIT DEFAULT 0,
    is_available BIT DEFAULT 1,
    created_at DATETIME2 DEFAULT GETDATE(),
    updated_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de atores
CREATE TABLE actors (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    biography TEXT,
    birth_date DATETIME2,
    profile_picture_url VARCHAR(500),
    created_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de diretores
CREATE TABLE directors (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    biography TEXT,
    birth_date DATETIME2,
    profile_picture_url VARCHAR(500),
    created_at DATETIME2 DEFAULT GETDATE()
);

-- =====================================================
-- TABELAS DE RELACIONAMENTO
-- =====================================================

-- Relacionamento vídeo-categoria
CREATE TABLE video_categories (
    id VARCHAR(36) PRIMARY KEY,
    video_id VARCHAR(36) NOT NULL,
    category_id VARCHAR(36) NOT NULL,
    created_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE,
    UNIQUE(video_id, category_id)
);

-- Relacionamento vídeo-ator
CREATE TABLE video_actors (
    id VARCHAR(36) PRIMARY KEY,
    video_id VARCHAR(36) NOT NULL,
    actor_id VARCHAR(36) NOT NULL,
    role_name VARCHAR(255),
    is_lead BIT DEFAULT 0,
    created_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
    FOREIGN KEY (actor_id) REFERENCES actors(id) ON DELETE CASCADE,
    UNIQUE(video_id, actor_id)
);

-- Relacionamento vídeo-diretor
CREATE TABLE video_directors (
    id VARCHAR(36) PRIMARY KEY,
    video_id VARCHAR(36) NOT NULL,
    director_id VARCHAR(36) NOT NULL,
    created_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
    FOREIGN KEY (director_id) REFERENCES directors(id) ON DELETE CASCADE,
    UNIQUE(video_id, director_id)
);

-- =====================================================
-- TABELAS DE FUNCIONALIDADE
-- =====================================================

-- Grupos de acesso
CREATE TABLE access_groups (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    permissions TEXT, -- JSON com permissões
    created_at DATETIME2 DEFAULT GETDATE()
);

-- Relacionamento usuário-grupo de acesso
CREATE TABLE users_access_groups (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    access_group_id VARCHAR(36) NOT NULL,
    assigned_at DATETIME2 DEFAULT GETDATE(),
    assigned_by VARCHAR(36),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (access_group_id) REFERENCES access_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_by) REFERENCES users(id),
    UNIQUE(user_id, access_group_id)
);

-- Histórico de visualização
CREATE TABLE watch_history (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    video_id VARCHAR(36) NOT NULL,
    watched_seconds INT DEFAULT 0,
    is_completed BIT DEFAULT 0,
    last_watched_at DATETIME2 DEFAULT GETDATE(),
    created_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
    UNIQUE(user_id, video_id)
);

-- Favoritos
CREATE TABLE favorites (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    video_id VARCHAR(36) NOT NULL,
    added_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
    UNIQUE(user_id, video_id)
);

-- Avaliações
CREATE TABLE ratings (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    video_id VARCHAR(36) NOT NULL,
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    created_at DATETIME2 DEFAULT GETDATE(),
    updated_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
    UNIQUE(user_id, video_id)
);

-- =====================================================
-- ÍNDICES PARA PERFORMANCE
-- =====================================================

-- Índices para usuários
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);

-- Índices para vídeos
CREATE INDEX idx_videos_title ON videos(title);
CREATE INDEX idx_videos_rating ON videos(rating);
CREATE INDEX idx_videos_featured ON videos(is_featured);
CREATE INDEX idx_videos_available ON videos(is_available);

-- Índices para relacionamentos
CREATE INDEX idx_video_categories_video ON video_categories(video_id);
CREATE INDEX idx_video_categories_category ON video_categories(category_id);
CREATE INDEX idx_video_actors_video ON video_actors(video_id);
CREATE INDEX idx_video_actors_actor ON video_actors(actor_id);
CREATE INDEX idx_video_directors_video ON video_directors(video_id);
CREATE INDEX idx_video_directors_director ON video_directors(director_id);

-- Índices para funcionalidades
CREATE INDEX idx_watch_history_user ON watch_history(user_id);
CREATE INDEX idx_watch_history_video ON watch_history(video_id);
CREATE INDEX idx_favorites_user ON favorites(user_id);
CREATE INDEX idx_favorites_video ON favorites(video_id);
CREATE INDEX idx_ratings_user ON ratings(user_id);
CREATE INDEX idx_ratings_video ON ratings(video_id);

-- =====================================================
-- DADOS INICIAIS
-- =====================================================

-- Inserir categorias padrão
INSERT INTO categories (id, name, description) VALUES
('cat-001', 'Ação', 'Filmes e séries de ação'),
('cat-002', 'Comédia', 'Filmes e séries de comédia'),
('cat-003', 'Drama', 'Filmes e séries dramáticos'),
('cat-004', 'Ficção Científica', 'Filmes e séries de ficção científica'),
('cat-005', 'Terror', 'Filmes e séries de terror'),
('cat-006', 'Romance', 'Filmes e séries românticos'),
('cat-007', 'Documentário', 'Documentários'),
('cat-008', 'Animação', 'Filmes e séries animados');

-- Inserir grupos de acesso padrão
INSERT INTO access_groups (id, name, description) VALUES
('group-001', 'Usuários Básicos', 'Acesso básico ao catálogo'),
('group-002', 'Usuários Premium', 'Acesso completo ao catálogo'),
('group-003', 'Administradores', 'Acesso total ao sistema');

-- Inserir usuário admin padrão
INSERT INTO users (id, email, password_hash, name, role) VALUES
('admin-001', 'admin@streaming.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/HS.iK8i', 'Admin', 'Admin');

-- Inserir usuário viewer padrão
INSERT INTO users (id, email, password_hash, name, role) VALUES
('viewer-001', 'viewer@streaming.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/HS.iK8i', 'Viewer', 'Viewer');

-- Associar usuários aos grupos
INSERT INTO users_access_groups (id, user_id, access_group_id) VALUES
('uag-001', 'admin-001', 'group-003'),
('uag-002', 'viewer-001', 'group-001');
