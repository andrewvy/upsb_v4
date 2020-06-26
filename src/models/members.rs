use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

#[derive(sqlx::FromRow)]
pub struct Member {
    /// ID
    pub userid: i64,
    /// Username
    pub username: String,
}

pub async fn get(db: &SqlitePool, id: i32) -> anyhow::Result<Member> {
    let member = sqlx::query_as(
        r#"
SELECT *
FROM user
WHERE userid = $1
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(member)
}
