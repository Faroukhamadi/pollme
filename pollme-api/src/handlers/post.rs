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
    select p.id, title, coalesce(sum(v.inc),0) as votes, coalesce(v.inc,0) as vote,
    count(c.id) as choice_count, p.created_at from post p left join vote v on p.id = v.post_id
    left join choice c on p.id = c.post_id
    where c.user_id is not null
    group by p.id, title, v.inc, p.created_at
        "#,
    )
    // select * from choice where post_id=1 and  user_id is not null;
    // this to fetch number of votes for choices later
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

pub(crate) async fn choice(
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
