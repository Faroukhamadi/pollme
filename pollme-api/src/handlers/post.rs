use axum::extract::{Path, Query};
use axum::{http::StatusCode, Extension};
use serde::{Deserialize, Serialize};
use sqlx::{types::Decimal, PgPool};

use crate::auth::Claims;
use crate::internal_error;

pub(crate) async fn posts(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
) -> Result<axum::Json<Vec<Post>>, (StatusCode, String)> {
    sqlx::query_as::<_, Post>(
        r#"
    select distinct on (p.id) p.id, title, coalesce(sum(v.inc), 0)  as votes, coalesce(v.inc,0) as vote,
    count(case when c.user_id is not null then c.id end) as choice_count, p.created_at from post p left join vote v on p.id = v.post_id
    left join choice c on p.id = c.post_id and c.user_id = p.user_id
    group by p.id, title, v.inc, p.created_at;
        "#,
    )
    .fetch_all(&pool)
    .await
    .map(|posts| axum::Json(posts))
    .map_err(internal_error)
}

pub(crate) async fn vote(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(post_id): Path<String>,
    vote: Query<VoteParam>,
) -> Result<axum::Json<i64>, (StatusCode, String)> {
    let vote_id = vote.id as i8;

    match vote_id.into() {
        VoteChoice::UpVote => {
            let row: Result<axum::Json<i64>, (axum::http::StatusCode, std::string::String)> =
                sqlx::query_as("select toggle_vote($1, $2, $3)")
                    .bind::<i64>(vote_id as i64)
                    .bind::<i64>(claims.sub.parse().unwrap())
                    .bind::<i64>(post_id.parse().unwrap())
                    .fetch_one(&pool)
                    .await
                    .map(|res: (i64,)| axum::Json(res.0))
                    .map_err(internal_error);
            row
        }
        VoteChoice::DownVote => {
            let row: Result<axum::Json<i64>, (axum::http::StatusCode, std::string::String)> =
                sqlx::query_as("select toggle_vote($1, $2, $3)")
                    .bind::<i64>(vote_id as i64)
                    .bind::<i64>(claims.sub.parse().unwrap())
                    .bind::<i64>(post_id.parse().unwrap())
                    .fetch_one(&pool)
                    .await
                    .map(|res: (i64,)| axum::Json(res.0))
                    .map_err(internal_error);
            row
        }
    }
}

pub(crate) async fn post_choices(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
    Path(post_id): Path<String>,
) -> Result<axum::Json<Vec<Choice>>, (StatusCode, String)> {
    let post_id = post_id.parse::<i16>().unwrap();
    sqlx::query_as::<_, Choice>(&format!(
        "select distinct on(name) name, id from choice where post_id = {post_id};"
    ))
    .fetch_all(&pool)
    .await
    .map(|choices| axum::Json(choices))
    .map_err(internal_error)
}

pub(crate) async fn user_choice(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
    Path((id, user_id)): Path<(String, String)>,
    // Path(user_id): Path<String>,
    // Path(id): Path<String>,
) -> Result<axum::Json<Option<Choice>>, (StatusCode, String)> {
    let user_id = user_id.parse::<i16>().unwrap();
    let id = id.parse::<i16>().unwrap();
    // TODO: fix this
    sqlx::query_as::<_, Choice>(&format!(
        "select distinct on(name) name, id from choice where id = {id} and user_id = {user_id};"
    ))
    .fetch_optional(&pool)
    .await
    .map(|choice| axum::Json(choice))
    .map_err(internal_error)
}

pub(crate) async fn _choice(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(post_id): Path<String>,
    choice: Query<ChoiceParam>,
) -> Result<axum::Json<i64>, (StatusCode, String)> {
    let choice = choice.num;

    match (choice as i8).into() {
        VoteChoice::UpVote => {
            let row: Result<axum::Json<i64>, (axum::http::StatusCode, std::string::String)> =
                sqlx::query_as("select toggle_vote($1, $2, $3)")
                    .bind::<i64>(choice)
                    .bind::<i64>(claims.sub.parse().unwrap())
                    .bind::<i64>(post_id.parse().unwrap())
                    .fetch_one(&pool)
                    .await
                    .map(|res: (i64,)| axum::Json(res.0))
                    .map_err(internal_error);
            row
        }
        VoteChoice::DownVote => {
            let row: Result<axum::Json<i64>, (axum::http::StatusCode, std::string::String)> =
                sqlx::query_as("select toggle_vote($1, $2, $3)")
                    .bind::<i64>(choice)
                    .bind::<i64>(claims.sub.parse().unwrap())
                    .bind::<i64>(post_id.parse().unwrap())
                    .fetch_one(&pool)
                    .await
                    .map(|res: (i64,)| axum::Json(res.0))
                    .map_err(internal_error);
            row
        }
    }
}

#[derive(Serialize, sqlx::FromRow, Debug)]
pub(crate) struct Post {
    id: i32,
    title: String,
    votes: Decimal,
    vote: i64,
    choice_count: i64,
    created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub(crate) struct VoteParam {
    id: i64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ChoiceParam {
    num: i64,
}

#[derive(Serialize, sqlx::FromRow, Debug)]
pub(crate) struct Vote {
    id: i32,
    inc: i64,
    created_at: chrono::NaiveDateTime,
    user_id: i64,
    post_id: i64,
}

#[derive(Serialize, sqlx::FromRow, Debug)]
pub(crate) struct Choice {
    name: String,
    id: i32,
    // user_id: Option<i32>,
}

pub(crate) enum VoteChoice {
    DownVote = -1,
    UpVote = 1,
}

impl From<i8> for VoteChoice {
    fn from(v: i8) -> Self {
        match v {
            x if x == VoteChoice::UpVote as i8 => VoteChoice::UpVote,
            x if x == VoteChoice::DownVote as i8 => VoteChoice::DownVote,
            _ => VoteChoice::UpVote,
        }
    }
}
