use auth::Keys;
use axum::{
    http::{HeaderValue, StatusCode},
    middleware,
    routing::{get, post},
    Extension, Router,
};

use dotenv::dotenv;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, SocketAddr};
use tower_http::cors::CorsLayer;

mod auth;
mod db;
mod handlers;
use auth::{auth, login, signup};
use handlers::post::{posts, vote};
use handlers::users::{create_user, users};

use crate::handlers::post::{
    choices_count, make_choice, post_choices, post_choices_count, user_choice, user_vote,
};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let host = std::env::var("DB_URL").expect("DB_URL must be set");
    let db_port = std::env::var("DB_PORT").expect("DB_PORT must be set");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");
    let server_port = std::env::var("PORT").expect("DEPLOY_PORT must be set");
    let server_port = server_port.parse::<u16>().unwrap();
    let ip_addr = if let Ok(ip) = std::env::var("IP_ADDR") {
        ip.parse::<IpAddr>().unwrap()
    } else {
        "0.0.0.0".parse::<IpAddr>().unwrap()
    };

    println!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgresql://postgres:{password}@{host}:{db_port}/{db_name}"
        ))
        .await?;

    println!("Connected to database");

    let with_auth = Router::new()
        .route("/posts", get(posts))
        .route("/users", get(users).post(create_user))
        .route("/posts/:post_id/vote", post(vote))
        .route("/posts/:post_id/choices", get(post_choices))
        .route("/choices/:name/:user_id", get(user_choice))
        // returns the number of choices given a name
        .route("/choices/:name/count", get(choices_count))
        // returns the total number of choices belonging to a post
        .route("/choices/count/:post_id", get(post_choices_count))
        .route("/choices", post(make_choice))
        .route("/vote", get(user_vote))
        .layer(middleware::from_fn(auth));

    let without_auth = Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup));

    let app = Router::new()
        .merge(with_auth)
        .merge(without_auth)
        .layer(Extension(pool))
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
        );

    let addr = SocketAddr::from((ip_addr, server_port));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
