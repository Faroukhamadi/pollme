use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{env::args, net::SocketAddr};

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

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS public.user (
    id serial NOT NULL,
    username character varying NOT NULL,
    password character varying NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);
    "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS public.post (
    id serial NOT NULL,
    title character varying NOT NULL,
    first_choice character varying NOT NULL,
    second_choice character varying NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT NOW(),
    user_id bigint,
    vote_id bigint,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES "user"(id)
);
    "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS public.vote (
    id serial NOT NULL,
    inc bigint NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT NOW(),
    user_id bigint,
    post_id bigint,
    check (inc in (-1, 1)),
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (post_id) REFERENCES post(id)
);
    "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"alter table public.post
add constraint post_votes FOREIGN KEY (vote_id) REFERENCES vote(id);"#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "
CREATE TABLE IF NOT EXISTS public.choice (
    id serial NOT NULL,
    choice character varying NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT NOW(),
    post_id bigint,
    PRIMARY KEY (id),
    FOREIGN KEY (post_id) REFERENCES post(id)
);
    ",
    )
    .execute(&pool)
    .await?;

    panic!("Done with migrations now I'm going to panic");

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
