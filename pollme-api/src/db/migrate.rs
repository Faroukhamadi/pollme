use sqlx::{Pool, Postgres};

pub async fn migrate_up(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
    .await?;

    sqlx::query(
        r#"alter table public.post
add constraint post_votes FOREIGN KEY (vote_id) REFERENCES vote(id);"#,
    )
    .execute(pool)
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
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn migrate_down(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let table_names = vec!["choice", "vote", "post", "user"];
    for table_name in table_names {
        sqlx::query(&format!(
            "DROP TABLE IF EXISTS public.{} CASCADE",
            table_name
        ))
        .execute(pool)
        .await?;
    }
    Ok(())
}
