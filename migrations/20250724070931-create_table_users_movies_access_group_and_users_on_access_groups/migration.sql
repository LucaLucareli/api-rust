CREATE TABLE users (
    id INT PRIMARY KEY IDENTITY(1,1),
    name NVARCHAR(255) NOT NULL,
    email NVARCHAR(255) NOT NULL,
    password_hash NVARCHAR(255) NOT NULL,
    created_at DATETIME2 NOT NULL
);

CREATE TABLE movies (
    id INT PRIMARY KEY IDENTITY(1,1),
    title NVARCHAR(255) NOT NULL,
    description NVARCHAR(MAX),
    release_year INT,
    duration_minutes INT,
    url NVARCHAR(2048),
    created_at DATETIME2 NOT NULL
);

CREATE TABLE access_group (
    id INT PRIMARY KEY IDENTITY(1,1),
    name NVARCHAR(255) NOT NULL,
    created_at DATETIME2 NOT NULL,
    updated_at DATETIME2 NOT NULL,
    deactivated_at DATETIME2 NULL
);

CREATE TABLE users_on_access_groups (
    user_id INT NOT NULL,
    access_group_id INT NOT NULL,
    PRIMARY KEY (user_id, access_group_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (access_group_id) REFERENCES access_group(id)
);

