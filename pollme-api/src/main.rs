use auth::Keys;
use axum::{
    http::{HeaderValue, StatusCode},
    middleware::{self},
    routing::{get, post},
    Extension, Router,
};
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use std::{env::args, net::SocketAddr};
use tower_http::cors::CorsLayer;
mod auth;
mod db;
mod handlers;
use auth::{auth, login};
use handlers::post::posts;
use handlers::users::{create_user, users};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

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

    let with_auth = Router::new()
        .route("/posts", get(posts))
        .route("/users", get(users).post(create_user))
        .layer(Extension(pool))
        // might add allow methods like this "allow_methods([Method::GET])""
        .layer(
            CorsLayer::new().allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
        )
        .route_layer(middleware::from_fn(auth));

    let without_auth = Router::new().route("/login", post(login)).layer(
        CorsLayer::new().allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
    );

    let app = Router::new().merge(with_auth).merge(without_auth);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

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
