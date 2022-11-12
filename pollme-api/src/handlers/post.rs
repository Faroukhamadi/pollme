use axum::{http::StatusCode, Extension};
use serde::Serialize;
use sqlx::{types::Decimal, PgPool};

use crate::auth::Claims;

#[derive(Serialize, sqlx::FromRow, Debug)]
pub(crate) struct Post {
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
    println!("posts is executed");
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
