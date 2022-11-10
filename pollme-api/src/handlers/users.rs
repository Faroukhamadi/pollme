use crate::internal_error;
use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub(crate) struct User {
    id: i32,
    username: String,
    password: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub(crate) struct CreateUser {
    username: String,
    password: String,
}

pub(crate) async fn users(
    Extension(pool): Extension<PgPool>,
) -> Result<axum::Json<Vec<User>>, (StatusCode, String)> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM "user";"#)
        .fetch_all(&pool)
        .await
        .map(|users| axum::Json(users))
        .map_err(internal_error)
}

// TODO: Hash entered password
pub(crate) async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<axum::Json<User>, (StatusCode, String)> {
    sqlx::query_as::<_, User>(&format!(
        "INSERT INTO public.user (username, password) VALUES('{}', '{}') RETURNING *;",
        payload.username, payload.password
    ))
    .fetch_one(&pool)
    .await
    .map(|user| axum::Json(user))
    .map_err(internal_error)
}
