use std::fmt::Display;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json, RequestPartsExt, TypedHeader,
};
use headers::{authorization::Bearer, Authorization, HeaderMap, HeaderName, HeaderValue};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::KEYS;

pub(crate) struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub(crate) fn new(secret: &[u8]) -> Self {
        Keys {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
pub(crate) enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        AuthBody {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
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

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Username: {}\nExpiry date: {}\ncompany: {}",
            self.sub, self.exp, self.company
        )
    }
}

pub(crate) async fn login(
    Json(payload): Json<AuthPayload>,
) -> (HeaderMap, Result<Json<AuthBody>, AuthError>) {
    let mut headers = HeaderMap::new();

    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return (headers, Err(AuthError::MissingCredentials));
    }
    if payload.client_id != "farouk" || payload.client_secret != "password123" {
        println!("they are different");
        return (headers, Err(AuthError::WrongCredentials));
    }
    let claims = Claims {
        sub: "farouk".to_owned(),
        company: "ISI Ariana".to_owned(),
        exp: 2000000000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)
        .expect("Unable to generate token");

    headers.insert(
        HeaderName::from_static("set-cookie"),
        HeaderValue::from_str(&format!(
            "sid={}; Max-Age=86400; Path=/; HttpOnly; Secure; SameSite=Strict",
            token
        ))
        .expect("Failed Setting headers"),
    );
    (headers, Ok(Json(AuthBody::new(token.to_string()))))
}

pub(crate) async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    println!("AUTH MIDDLEWARE RUN");

    let cookie_header = req
        .headers()
        .get(header::COOKIE)
        .and_then(|header| header.to_str().ok());

    println!("cookie header: {:?}", req.headers());

    let cookie = if let Some(cookie_header) = cookie_header {
        println!("cookie_header: {:?}", cookie_header);
        cookie_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some((_, token_val)) = cookie.split_once("=") {
        let token_data = decode::<Claims>(token_val, &KEYS.decoding, &Validation::default());
        if let Ok(token_data) = token_data {
            req.extensions_mut().insert(token_data.claims);
            println!("we are going next");
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}