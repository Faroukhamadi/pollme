use axum::extract::{Path, Query};
use axum::{http::StatusCode, Extension};
use serde::{Deserialize, Serialize};
use sqlx::{types::Decimal, PgPool};

use crate::auth::Claims;

#[derive(Serialize, sqlx::FromRow, Debug)]
pub(crate) struct Post {
    id: i32,
    title: String,
    first_choice: String,
    second_choice: String,
    votes: Decimal,
    first_choice_count: i64,
    second_choice_count: i64,
    choice_count: i32,
    created_at: chrono::NaiveDateTime,
}
use crate::internal_error;

pub(crate) async fn posts(
    Extension(pool): Extension<PgPool>,
    Extension(_): Extension<Claims>,
) -> Result<axum::Json<Vec<Post>>, (StatusCode, String)> {
    sqlx::query_as::<_, Post>(
        r#"
    select p.id, title, p.first_choice, p.second_choice, sum(v.inc) as votes,
    sum(p.first_choice_count) as first_choice_count, sum(p.second_choice_count) as
    second_choice_count, (p.first_choice_count + p.second_choice_count) as choice_count,
    p.created_at from post p inner join vote v on p.id = v.post_id group by p.id, title,
    p.first_choice, p.second_choice, p.created_at, choice_count order by choice_count desc,
    p.created_at desc;
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
) -> Result<axum::Json<Vote>, (StatusCode, String)> {
    let vote_id = vote.id as i8;

    match vote_id.into() {
        VoteChoice::UpVote => sqlx::query_as::<_, Vote>(&format!(
            "insert into vote (inc, user_id, post_id) values('{}', '{}', '{}') returning *;",
            vote_id, claims.sub, post_id
        ))
        .fetch_one(&pool)
        .await
        .map(|user| axum::Json(user))
        .map_err(internal_error),
        VoteChoice::DownVote => sqlx::query_as::<_, Vote>(&format!(
            "insert into vote (inc, user_id, post_id) values('{}', '{}', '{}') returning *;",
            vote_id, claims.sub, post_id
        ))
        .fetch_one(&pool)
        .await
        .map(|user| axum::Json(user))
        .map_err(internal_error),
        VoteChoice::RemoveVote => sqlx::query_as::<_, Vote>(&format!(
            "delete from vote where user_id='{}' and post_id='{}' returning *;",
            vote_id, claims.sub
        ))
        .fetch_one(&pool)
        .await
        .map(|user| axum::Json(user))
        .map_err(internal_error),
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct VoteParam {
    id: i8,
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
    RemoveVote,
    UpVote,
}

impl From<i8> for VoteChoice {
    fn from(v: i8) -> Self {
        match v {
            x if x == VoteChoice::UpVote as i8 => VoteChoice::UpVote,
            x if x == VoteChoice::DownVote as i8 => VoteChoice::DownVote,
            x if x == VoteChoice::RemoveVote as i8 => VoteChoice::RemoveVote,
            _ => VoteChoice::RemoveVote,
        }
    }
}
