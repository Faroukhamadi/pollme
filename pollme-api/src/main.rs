use axum::{
    http::{HeaderValue, StatusCode},
    routing::get,
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPoolOptions,
    types::{chrono, Decimal},
    PgPool,
};
use std::{env::args, net::SocketAddr};
use tower_http::cors::{Cors, CorsLayer};
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
        .route("/", get(posts))
        .route("/users", get(users).post(create_user))
        .layer(Extension(pool))
        // might add allow methods like this "allow_methods([Method::GET])""
        .layer(
            CorsLayer::new().allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
        );
    // .route("/users", get(users));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// might change this to order by votes and then date
async fn posts(
    Extension(pool): Extension<PgPool>,
) -> Result<axum::Json<Vec<Post>>, (StatusCode, String)> {
    sqlx::query_as::<_, Post>(
        r#"
select title, p.first_choice, p.second_choice, sum(v.inc) as votes,
sum(p.first_choice_count) as first_choice_count, sum(p.second_choice_count) as
second_choice_count, (p.first_choice_count + p.second_choice_count) as choice_count,
p.created_at from post p inner join vote v on p.id = v.post_id group by title,
p.first_choice, p.second_choice, p.created_at, choice_count order by choice_count desc,
p.created_at desc;
    "#,
    )
    .fetch_all(&pool)
    .await
    .map(|posts| axum::Json(posts))
    .map_err(internal_error)
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

// TODO: Hash entered password
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

#[derive(Serialize, sqlx::FromRow)]
struct Post {
    title: String,
    first_choice: String,
    second_choice: String,
    votes: Decimal,
    first_choice_count: i64,
    second_choice_count: i64,
    choice_count: i32,
    created_at: chrono::NaiveDateTime,
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
