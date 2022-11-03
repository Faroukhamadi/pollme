use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{env::args, net::SocketAddr};
mod db;
// use crate::db::migrate::migrate_thingie;
use db::migrate::{migrate_down, migrate_up};
use db::seed::seed_users;

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

    seed_users(pool).await?;

    // migrate_up(&pool).await?;

    panic!("done migrating up");

    let (code,): (String,) = sqlx::query_as("SELECT code from country where name = 'Aruba'")
        .fetch_one(&pool)
        .await?;

    println!("code is: {}", code);

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

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

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    let u = Json(user);
    println!("id: {}, username: {}", u.id, u.username);

    (StatusCode::CREATED, u)
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
