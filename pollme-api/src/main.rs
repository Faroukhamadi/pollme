use ::chrono::{NaiveDate, NaiveDateTime};
use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::chrono, PgPool};
use std::{env::args, net::SocketAddr};
mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut args = args();

    args.next();

    let password = args.next();
    if password.is_none() {
        panic!("Please enter database password as first argument");
    }
    let password = password.unwrap();

    if password.len() == 0 {
        panic!("Please enter password as argument")
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgres://postgres:{password}@localhost:5432/pollme",
        ))
        .await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(users).post(create_user))
        .layer(Extension(pool));
    // .route("/users", get(users));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn users(
    Extension(pool): Extension<PgPool>,
) -> Result<axum::Json<Vec<User>>, (StatusCode, String)> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM "user";"#)
        .fetch_all(&pool)
        .await
        .map(|users| axum::Json(users))
        .map_err(internal_error)
}

async fn create_user(
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

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    password: String,
    created_at: chrono::NaiveDateTime,
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
