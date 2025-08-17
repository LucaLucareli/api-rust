use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::Bearer;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::database::repositories::users::UsersRepository;
use crate::modules::jwt::JwtManager;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub async fn auth(
        State(db): State<DatabaseConnection>,
        State(jwt_manager): State<JwtManager>,
        bearer: Option<Bearer>,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let token = bearer
            .ok_or(StatusCode::UNAUTHORIZED)?
            .token();

        // Validar token
        let claims = jwt_manager
            .validate_access_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Buscar usuário no banco
        let users_repo = UsersRepository::new(db);
        let user = users_repo
            .find_by_id(&claims.sub)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Adicionar usuário ao request extensions
        request.extensions_mut().insert(user);
        request.extensions_mut().insert(claims);

        Ok(next.run(request).await)
    }
}

pub struct AdminMiddleware;

impl AdminMiddleware {
    pub async fn admin(
        State(db): State<DatabaseConnection>,
        State(jwt_manager): State<JwtManager>,
        bearer: Option<Bearer>,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let token = bearer
            .ok_or(StatusCode::UNAUTHORIZED)?
            .token();

        // Validar token
        let claims = jwt_manager
            .validate_access_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Buscar usuário no banco
        let users_repo = UsersRepository::new(db);
        let user = users_repo
            .find_by_id(&claims.sub)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Verificar se é admin
        if user.role != "admin" {
            return Err(StatusCode::FORBIDDEN);
        }

        // Adicionar usuário ao request extensions
        request.extensions_mut().insert(user);
        request.extensions_mut().insert(claims);

        Ok(next.run(request).await)
    }
}
