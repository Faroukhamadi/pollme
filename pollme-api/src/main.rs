use axum::{
    http::StatusCode, response::IntoResponse, routing::get, routing::post, Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::chrono, PgPool, Pool, Postgres};
use std::{env::args, net::SocketAddr, time};
mod db;
use crate::db::seed::seed_vote;
use db::seed::{seed_posts, seed_users};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut args = args();

    // remove first argument since it's path
    args.next();

    // we enter db password as second argument
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
        .route("/users", get(users))
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

// async fn create_user(Extension(app_state): Extension<AppState>) -> impl IntoResponse {
//     let user = User {
//         id: 1337,
//         username: "hello".to_string(),
//         created_at: "".to_string(),
//         password: "".to_string(),
//     };

//     let u = Json(user);
//     println!("id: {}, username: {}", u.id, u.username);

//     (StatusCode::CREATED, u)
// }

#[derive(Deserialize)]
struct CreateUser {
    username: String,
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
