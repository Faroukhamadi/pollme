use fake::{
    faker::{
        internet::en::Password,
        lorem::en::{Sentence, Word},
        name::en::Name,
    },
    Fake,
};
use sqlx::{Pool, Postgres};

pub async fn _seed_users(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    for _ in 1..=10 {
        sqlx::query(&format!(
            "INSERT INTO public.user (username, password) VALUES('{}', '{}')",
            Name().fake::<String>(),
            Password(5..8).fake::<String>()
        ))
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn _seed_choices(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    for user_id in 1..=10 {
        sqlx::query(&format!(
            "INSERT INTO public.choice (name, user_id, post_id) VALUES('{}', '{}', '{}')",
            Word().fake::<String>(),
            user_id,
            user_id
        ))
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn _seed_posts(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    for user_id in 1..=10 {
        sqlx::query(&format!(
            "INSERT INTO public.post (title, user_id) VALUES('{}', '{}')",
            Sentence(10..20).fake::<String>(),
            user_id,
        ))
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn _seed_votes(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    for post_and_user_id in 1..=10 {
        let inc;
        if post_and_user_id % 2 == 0 {
            inc = 1
        } else {
            inc = -1
        }
        sqlx::query(&format!(
            "INSERT INTO public.vote (inc, user_id, post_id) VALUES({}, {}, {})",
            inc, post_and_user_id, post_and_user_id
        ))
        .execute(pool)
        .await?;
    }

    Ok(())
}
