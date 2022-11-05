use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, RequestPartsExt, Router, TypedHeader,
};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{
    postgres::PgPoolOptions,
    types::{chrono, Decimal},
    PgPool,
};
use std::{env::args, fmt::Display, net::SocketAddr};
use tower_http::cors::CorsLayer;
mod db;

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

    let app = Router::new()
        .route("/", get(posts))
        .route("/users", get(users).post(create_user))
        .route("/protected", get(protected))
        .route("/authorize", post(authorize))
        .layer(Extension(pool))
        // might add allow methods like this "allow_methods([Method::GET])""
        .layer(
            CorsLayer::new().allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
        );

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

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area, Your data:\n{}",
        claims
    ))
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

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    if payload.client_id != "farouk" || payload.client_secret != "password123" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: "farouk".to_owned(),
        company: "ISI Ariana".to_owned(),
        exp: 2000000000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody::new(token)))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}\nExpiry date: {}", self.sub, self.exp)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        AuthBody {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
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

// HERE
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode user data
        let token_data = decode(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Keys {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
