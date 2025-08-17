-- Migração para sistema de streaming de vídeos
-- Criando tabelas para um sistema tipo Netflix

-- Tabela de usuários
CREATE TABLE users (
    id NVARCHAR(36) PRIMARY KEY,
    email NVARCHAR(255) UNIQUE NOT NULL,
    password_hash NVARCHAR(255) NOT NULL,
    name NVARCHAR(255) NOT NULL,
    profile_picture_url NVARCHAR(500),
    subscription_status NVARCHAR(50) DEFAULT 'active',
    subscription_expires_at DATETIME2,
    created_at DATETIME2 DEFAULT GETDATE(),
    updated_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de categorias/genres
CREATE TABLE categories (
    id NVARCHAR(36) PRIMARY KEY,
    name NVARCHAR(100) NOT NULL,
    description NVARCHAR(500),
    created_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de vídeos
CREATE TABLE videos (
    id NVARCHAR(36) PRIMARY KEY,
    title NVARCHAR(255) NOT NULL,
    description NVARCHAR(1000),
    duration_seconds INT NOT NULL,
    release_year INT,
    rating DECIMAL(3,1) DEFAULT 0.0,
    thumbnail_url NVARCHAR(500),
    video_url NVARCHAR(500),
    trailer_url NVARCHAR(500),
    is_featured BIT DEFAULT 0,
    is_available BIT DEFAULT 1,
    created_at DATETIME2 DEFAULT GETDATE(),
    updated_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de relacionamento vídeo-categoria
CREATE TABLE video_categories (
    id NVARCHAR(36) PRIMARY KEY,
    video_id NVARCHAR(36) NOT NULL,
    category_id NVARCHAR(36) NOT NULL,
    FOREIGN KEY (video_id) REFERENCES videos(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- Tabela de atores
CREATE TABLE actors (
    id NVARCHAR(36) PRIMARY KEY,
    name NVARCHAR(255) NOT NULL,
    biography NVARCHAR(1000),
    birth_date DATE,
    profile_picture_url NVARCHAR(500),
    created_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de relacionamento vídeo-ator
CREATE TABLE video_actors (
    id NVARCHAR(36) PRIMARY KEY,
    video_id NVARCHAR(36) NOT NULL,
    actor_id NVARCHAR(36) NOT NULL,
    role_name NVARCHAR(255),
    is_lead BIT DEFAULT 0,
    FOREIGN KEY (video_id) REFERENCES videos(id),
    FOREIGN KEY (actor_id) REFERENCES actors(id)
);

-- Tabela de diretores
CREATE TABLE directors (
    id NVARCHAR(36) PRIMARY KEY,
    name NVARCHAR(255) NOT NULL,
    biography NVARCHAR(1000),
    birth_date DATE,
    profile_picture_url NVARCHAR(500),
    created_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de relacionamento vídeo-diretor
CREATE TABLE video_directors (
    id NVARCHAR(36) PRIMARY KEY,
    video_id NVARCHAR(36) NOT NULL,
    director_id NVARCHAR(36) NOT NULL,
    FOREIGN KEY (video_id) REFERENCES videos(id),
    FOREIGN KEY (director_id) REFERENCES directors(id)
);

-- Tabela de grupos de acesso
CREATE TABLE access_groups (
    id NVARCHAR(36) PRIMARY KEY,
    name NVARCHAR(100) NOT NULL,
    description NVARCHAR(500),
    permissions NVARCHAR(MAX), -- JSON com permissões
    created_at DATETIME2 DEFAULT GETDATE()
);

-- Tabela de relacionamento usuário-grupo de acesso
CREATE TABLE users_access_groups (
    id NVARCHAR(36) PRIMARY KEY,
    user_id NVARCHAR(36) NOT NULL,
    access_group_id NVARCHAR(36) NOT NULL,
    assigned_at DATETIME2 DEFAULT GETDATE(),
    assigned_by NVARCHAR(36),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (access_group_id) REFERENCES access_groups(id),
    FOREIGN KEY (assigned_by) REFERENCES users(id)
);

-- Tabela de histórico de visualização
CREATE TABLE watch_history (
    id NVARCHAR(36) PRIMARY KEY,
    user_id NVARCHAR(36) NOT NULL,
    video_id NVARCHAR(36) NOT NULL,
    watched_seconds INT DEFAULT 0,
    is_completed BIT DEFAULT 0,
    last_watched_at DATETIME2 DEFAULT GETDATE(),
    created_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (video_id) REFERENCES videos(id)
);

-- Tabela de favoritos
CREATE TABLE favorites (
    id NVARCHAR(36) PRIMARY KEY,
    user_id NVARCHAR(36) NOT NULL,
    video_id NVARCHAR(36) NOT NULL,
    added_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (video_id) REFERENCES videos(id),
    UNIQUE(user_id, video_id)
);

-- Tabela de avaliações
CREATE TABLE ratings (
    id NVARCHAR(36) PRIMARY KEY,
    user_id NVARCHAR(36) NOT NULL,
    video_id NVARCHAR(36) NOT NULL,
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment NVARCHAR(1000),
    created_at DATETIME2 DEFAULT GETDATE(),
    updated_at DATETIME2 DEFAULT GETDATE(),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (video_id) REFERENCES videos(id),
    UNIQUE(user_id, video_id)
);

-- Inserir dados de exemplo
INSERT INTO categories (id, name, description) VALUES
('cat-001', 'Ação', 'Filmes de ação e aventura'),
('cat-002', 'Comédia', 'Filmes de comédia'),
('cat-003', 'Drama', 'Filmes dramáticos'),
('cat-004', 'Ficção Científica', 'Filmes de ficção científica'),
('cat-005', 'Terror', 'Filmes de terror e suspense');

INSERT INTO access_groups (id, name, description) VALUES
('group-001', 'Admin', 'Administradores do sistema'),
('group-002', 'Moderator', 'Moderadores de conteúdo'),
('group-003', 'Premium', 'Usuários premium'),
('group-004', 'Standard', 'Usuários padrão');

-- Criar índices para melhor performance
CREATE INDEX idx_videos_title ON videos(title);
CREATE INDEX idx_videos_category ON video_categories(category_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_watch_history_user ON watch_history(user_id);
CREATE INDEX idx_favorites_user ON favorites(user_id);

