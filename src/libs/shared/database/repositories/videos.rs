use sea_orm::{DatabaseConnection, Statement, FromQueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
    pub duration_seconds: i32,
    pub release_year: Option<i32>,
    pub rating: f64,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
    pub is_featured: bool,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: String,
    pub duration_seconds: i32,
    pub release_year: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVideoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_seconds: Option<i32>,
    pub release_year: Option<i32>,
    pub rating: Option<f64>,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
    pub is_featured: Option<bool>,
    pub is_available: Option<bool>,
}

pub struct VideosRepository {
    db: DatabaseConnection,
}

impl VideosRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, request: CreateVideoRequest) -> Result<Video, sea_orm::DbErr> {
        let video_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let sql = r#"
            INSERT INTO videos (id, title, description, duration_seconds, release_year, 
                               rating, thumbnail_url, video_url, trailer_url, 
                               is_featured, is_available, created_at, updated_at)
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11, @P12, @P13)
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![
                video_id.into(),
                request.title.into(),
                request.description.into(),
                request.duration_seconds.into(),
                request.release_year.into(),
                0.0.into(), // rating inicial
                request.thumbnail_url.into(),
                request.video_url.into(),
                request.trailer_url.into(),
                false.into(), // is_featured inicial
                true.into(),  // is_available inicial
                now.into(),
                now.into(),
            ],
        );

        self.db.execute(stmt).await?;

        // Buscar vídeo criado
        self.find_by_id(&video_id).await
            .and_then(|video| video.ok_or(sea_orm::DbErr::Custom("Vídeo não encontrado após criação".to_string())))
    }

    pub async fn find_by_id(&self, video_id: &str) -> Result<Option<Video>, sea_orm::DbErr> {
        let sql = r#"
            SELECT id, title, description, duration_seconds, release_year, 
                   rating, thumbnail_url, video_url, trailer_url, 
                   is_featured, is_available, created_at, updated_at
            FROM videos
            WHERE id = @P1
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![video_id.into()],
        );

        let result = self.db.query_one(stmt).await?;
        
        if let Some(row) = result {
            let id: String = row.try_get("", "id").unwrap_or_default();
            let title: String = row.try_get("", "title").unwrap_or_default();
            let description: String = row.try_get("", "description").unwrap_or_default();
            let duration_seconds: i32 = row.try_get("", "duration_seconds").unwrap_or(0);
            let release_year: Option<i32> = row.try_get("", "release_year").ok();
            let rating: f64 = row.try_get("", "rating").unwrap_or(0.0);
            let thumbnail_url: Option<String> = row.try_get("", "thumbnail_url").ok();
            let video_url: Option<String> = row.try_get("", "video_url").ok();
            let trailer_url: Option<String> = row.try_get("", "trailer_url").ok();
            let is_featured: bool = row.try_get("", "is_featured").unwrap_or(false);
            let is_available: bool = row.try_get("", "is_available").unwrap_or(true);
            let created_at: DateTime<Utc> = row.try_get("", "created_at").unwrap_or_else(|_| Utc::now());
            let updated_at: DateTime<Utc> = row.try_get("", "updated_at").unwrap_or_else(|_| Utc::now());

            Ok(Some(Video {
                id,
                title,
                description,
                duration_seconds,
                release_year,
                rating,
                thumbnail_url,
                video_url,
                trailer_url,
                is_featured,
                is_available,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all(&self, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<Video>, sea_orm::DbErr> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let sql = r#"
            SELECT id, title, description, duration_seconds, release_year, 
                   rating, thumbnail_url, video_url, trailer_url, 
                   is_featured, is_available, created_at, updated_at
            FROM videos
            WHERE is_available = 1
            ORDER BY created_at DESC
            OFFSET @P1 ROWS
            FETCH NEXT @P2 ROWS ONLY
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![offset.into(), limit.into()],
        );

        let results = self.db.query_all(stmt).await?;
        
        let mut videos = Vec::new();
        for row in results {
            let id: String = row.try_get("", "id").unwrap_or_default();
            let title: String = row.try_get("", "title").unwrap_or_default();
            let description: String = row.try_get("", "description").unwrap_or_default();
            let duration_seconds: i32 = row.try_get("", "duration_seconds").unwrap_or(0);
            let release_year: Option<i32> = row.try_get("", "release_year").ok();
            let rating: f64 = row.try_get("", "rating").unwrap_or(0.0);
            let thumbnail_url: Option<String> = row.try_get("", "thumbnail_url").ok();
            let video_url: Option<String> = row.try_get("", "video_url").ok();
            let trailer_url: Option<String> = row.try_get("", "trailer_url").ok();
            let is_featured: bool = row.try_get("", "is_featured").unwrap_or(false);
            let is_available: bool = row.try_get("", "is_available").unwrap_or(true);
            let created_at: DateTime<Utc> = row.try_get("", "created_at").unwrap_or_else(|_| Utc::now());
            let updated_at: DateTime<Utc> = row.try_get("", "updated_at").unwrap_or_else(|_| Utc::now());

            videos.push(Video {
                id,
                title,
                description,
                duration_seconds,
                release_year,
                rating,
                thumbnail_url,
                video_url,
                trailer_url,
                is_featured,
                is_available,
                created_at,
                updated_at,
            });
        }

        Ok(videos)
    }

    pub async fn find_featured(&self, limit: Option<u64>) -> Result<Vec<Video>, sea_orm::DbErr> {
        let limit = limit.unwrap_or(10);

        let sql = r#"
            SELECT id, title, description, duration_seconds, release_year, 
                   rating, thumbnail_url, video_url, trailer_url, 
                   is_featured, is_available, created_at, updated_at
            FROM videos
            WHERE is_featured = 1 AND is_available = 1
            ORDER BY rating DESC, created_at DESC
            OFFSET 0 ROWS
            FETCH NEXT @P1 ROWS ONLY
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![limit.into()],
        );

        let results = self.db.query_all(stmt).await?;
        
        let mut videos = Vec::new();
        for row in results {
            let id: String = row.try_get("", "id").unwrap_or_default();
            let title: String = row.try_get("", "title").unwrap_or_default();
            let description: String = row.try_get("", "description").unwrap_or_default();
            let duration_seconds: i32 = row.try_get("", "duration_seconds").unwrap_or(0);
            let release_year: Option<i32> = row.try_get("", "release_year").ok();
            let rating: f64 = row.try_get("", "rating").unwrap_or(0.0);
            let thumbnail_url: Option<String> = row.try_get("", "thumbnail_url").ok();
            let video_url: Option<String> = row.try_get("", "video_url").ok();
            let trailer_url: Option<String> = row.try_get("", "trailer_url").ok();
            let is_featured: bool = row.try_get("", "is_featured").unwrap_or(false);
            let is_available: bool = row.try_get("", "is_available").unwrap_or(true);
            let created_at: DateTime<Utc> = row.try_get("", "created_at").unwrap_or_else(|_| Utc::now());
            let updated_at: DateTime<Utc> = row.try_get("", "updated_at").unwrap_or_else(|_| Utc::now());

            videos.push(Video {
                id,
                title,
                description,
                duration_seconds,
                release_year,
                rating,
                thumbnail_url,
                video_url,
                trailer_url,
                is_featured,
                is_available,
                created_at,
                updated_at,
            });
        }

        Ok(videos)
    }

    pub async fn find_by_title(&self, title: &str, limit: Option<u64>) -> Result<Vec<Video>, sea_orm::DbErr> {
        let limit = limit.unwrap_or(20);

        let sql = r#"
            SELECT id, title, description, duration_seconds, release_year, 
                   rating, thumbnail_url, video_url, trailer_url, 
                   is_featured, is_available, created_at, updated_at
            FROM videos
            WHERE title LIKE @P1 AND is_available = 1
            ORDER BY rating DESC, created_at DESC
            OFFSET 0 ROWS
            FETCH NEXT @P2 ROWS ONLY
        "#;

        let search_term = format!("%{}%", title);

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![search_term.into(), limit.into()],
        );

        let results = self.db.query_all(stmt).await?;
        
        let mut videos = Vec::new();
        for row in results {
            let id: String = row.try_get("", "id").unwrap_or_default();
            let title: String = row.try_get("", "title").unwrap_or_default();
            let description: String = row.try_get("", "description").unwrap_or_default();
            let duration_seconds: i32 = row.try_get("", "duration_seconds").unwrap_or(0);
            let release_year: Option<i32> = row.try_get("", "release_year").ok();
            let rating: f64 = row.try_get("", "rating").unwrap_or(0.0);
            let thumbnail_url: Option<String> = row.try_get("", "thumbnail_url").ok();
            let video_url: Option<String> = row.try_get("", "video_url").ok();
            let trailer_url: Option<String> = row.try_get("", "trailer_url").ok();
            let is_featured: bool = row.try_get("", "is_featured").unwrap_or(false);
            let is_available: bool = row.try_get("", "is_available").unwrap_or(true);
            let created_at: DateTime<Utc> = row.try_get("", "created_at").unwrap_or_else(|_| Utc::now());
            let updated_at: DateTime<Utc> = row.try_get("", "updated_at").unwrap_or_else(|_| Utc::now());

            videos.push(Video {
                id,
                title,
                description,
                duration_seconds,
                release_year,
                rating,
                thumbnail_url,
                video_url,
                trailer_url,
                is_featured,
                is_available,
                created_at,
                updated_at,
            });
        }

        Ok(videos)
    }

    pub async fn update(&self, video_id: &str, request: UpdateVideoRequest) -> Result<Option<Video>, sea_orm::DbErr> {
        let now = Utc::now();
        let mut updates = Vec::new();
        let mut params = Vec::new();
        let mut param_count = 1;

        if let Some(title) = &request.title {
            updates.push(format!("title = @P{}", param_count));
            params.push(title.into());
            param_count += 1;
        }

        if let Some(description) = &request.description {
            updates.push(format!("description = @P{}", param_count));
            params.push(description.into());
            param_count += 1;
        }

        if let Some(duration_seconds) = &request.duration_seconds {
            updates.push(format!("duration_seconds = @P{}", param_count));
            params.push(duration_seconds.into());
            param_count += 1;
        }

        if let Some(release_year) = &request.release_year {
            updates.push(format!("release_year = @P{}", param_count));
            params.push(release_year.into());
            param_count += 1;
        }

        if let Some(rating) = &request.rating {
            updates.push(format!("rating = @P{}", param_count));
            params.push(rating.into());
            param_count += 1;
        }

        if let Some(thumbnail_url) = &request.thumbnail_url {
            updates.push(format!("thumbnail_url = @P{}", param_count));
            params.push(thumbnail_url.into());
            param_count += 1;
        }

        if let Some(video_url) = &request.video_url {
            updates.push(format!("video_url = @P{}", param_count));
            params.push(video_url.into());
            param_count += 1;
        }

        if let Some(trailer_url) = &request.trailer_url {
            updates.push(format!("trailer_url = @P{}", param_count));
            params.push(trailer_url.into());
            param_count += 1;
        }

        if let Some(is_featured) = &request.is_featured {
            updates.push(format!("is_featured = @P{}", param_count));
            params.push(is_featured.into());
            param_count += 1;
        }

        if let Some(is_available) = &request.is_available {
            updates.push(format!("is_available = @P{}", param_count));
            params.push(is_available.into());
            param_count += 1;
        }

        updates.push(format!("updated_at = @P{}", param_count));
        params.push(now.into());
        param_count += 1;

        if updates.is_empty() {
            return self.find_by_id(video_id).await;
        }

        let sql = format!(
            "UPDATE videos SET {} WHERE id = @P{}",
            updates.join(", "),
            param_count
        );

        params.push(video_id.into());

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            params,
        );

        self.db.execute(stmt).await?;

        // Buscar vídeo atualizado
        self.find_by_id(video_id).await
    }

    pub async fn delete(&self, video_id: &str) -> Result<bool, sea_orm::DbErr> {
        let sql = "DELETE FROM videos WHERE id = @P1";

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![video_id.into()],
        );

        let result = self.db.execute(stmt).await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn count(&self) -> Result<u64, sea_orm::DbErr> {
        let sql = "SELECT COUNT(*) as count FROM videos WHERE is_available = 1";

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![],
        );

        let result = self.db.query_one(stmt).await?;
        
        if let Some(row) = result {
            let count: i64 = row.try_get("", "count").unwrap_or(0);
            Ok(count as u64)
        } else {
            Ok(0)
        }
    }
}
