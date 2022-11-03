use fake::{
    faker::{
        internet::en::Password,
        lorem::en::{Sentence, Word},
        name::en::Name,
    },
    Fake,
};
use sqlx::{Pool, Postgres};

pub async fn seed_users(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
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

pub async fn seed_posts(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    for user_id in 1..=10 {
        sqlx::query(&format!(
            "INSERT INTO public.post (title, first_choice, second_choice, user_id) VALUES('{}', '{}', '{}', {})",
            Sentence(10..20).fake::<String>(),
            Word().fake::<String>(),
            Word().fake::<String>(),
            user_id
        ))
        .execute(pool)
        .await?;
    }

    Ok(())
}
