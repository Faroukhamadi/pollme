use fake::{
    faker::{internet::en::Password, name::en::Name},
    Fake,
};
use sqlx::{Pool, Postgres};

pub async fn seed_users(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    for _ in 1..=10 {
        sqlx::query(&format!(
            "INSERT INTO public.user (username, password) VALUES('{}', '{}')",
            Name().fake::<String>(),
            Password(5..8).fake::<String>()
        ))
        .execute(&pool)
        .await?;
    }

    Ok(())
}
