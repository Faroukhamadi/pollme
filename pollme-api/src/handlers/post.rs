use axum::extract::{Path, Query};
use axum::{http::StatusCode, Extension};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::Claims;
use crate::internal_error;

pub(crate) async fn posts(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
) -> Result<axum::Json<Vec<Post>>, (StatusCode, String)> {
    sqlx::query_as::<_, Post>(
        r#"
    select distinct on (p.id) p.id, title, coalesce(sum(v.inc), 0)::int as votes, coalesce(v.inc,0) as vote,
     p.created_at from post p left join vote v on p.id = v.post_id
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
        "select distinct on(name) name, id, post_id from choice where post_id = {post_id};"
    ))
    .fetch_all(&pool)
    .await
    .map(|choices| axum::Json(choices))
    .map_err(internal_error)
}

pub(crate) async fn post_choices_count(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
    Path(post_id): Path<String>,
) -> Result<axum::Json<i64>, (StatusCode, String)> {
    sqlx::query_as(&format!(
        "select count(*) from choice where post_id = {}  and user_id is not null;",
        post_id
    ))
    .fetch_one(&pool)
    .await
    .map(|choices: (i64,)| axum::Json(choices.0))
    .map_err(internal_error)
}

pub(crate) async fn choices_count(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
    Path(name): Path<String>,
) -> Result<axum::Json<i64>, (StatusCode, String)> {
    sqlx::query_as(&format!(
        "select count(*) from choice where name = '{}' and user_id is not null;",
        name
    ))
    .fetch_one(&pool)
    .await
    .map(|choices: (i64,)| axum::Json(choices.0))
    .map_err(internal_error)
}

pub(crate) async fn make_choice(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
    choice: Query<MakeChoiceParam>,
) -> Result<axum::Json<Choice>, (StatusCode, String)> {
    sqlx::query_as(&format!(
        "insert into choice (name, post_id, user_id) values('{}', {}, {}) returning name, id, post_id;",
        choice.name, choice.post_id, choice.user_id,
    ))
    .fetch_one(&pool)
    .await
    .map(|choice| axum::Json(choice))
    .map_err(internal_error)
}

pub(crate) async fn user_vote(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
    vote: Query<UserVoteParam>,
) -> Result<axum::Json<i64>, (StatusCode, String)> {
    sqlx::query_as(&format!(
        "select coalesce((select inc from vote where post_id = {} and user_id = {}), 0)",
        vote.post_id, vote.user_id
    ))
    .fetch_one(&pool)
    .await
    .map(|choice: (i64,)| axum::Json(choice.0))
    .map_err(internal_error)
}

pub(crate) async fn user_choice(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
    Path((name, user_id)): Path<(String, String)>,
) -> Result<axum::Json<Option<Choice>>, (StatusCode, String)> {
    let user_id = user_id.parse::<i16>().unwrap();
    sqlx::query_as::<_, Choice>(&format!(
        "select distinct on(name) name, id, post_id from choice where name = '{}' and user_id = {};",
        name, user_id
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
    votes: i32,
    created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub(crate) struct VoteParam {
    id: i64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MakeChoiceParam {
    name: String,
    post_id: i64,
    user_id: i64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserVoteParam {
    post_id: i64,
    user_id: i64,
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
    post_id: i32,
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
