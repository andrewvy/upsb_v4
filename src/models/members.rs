use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

#[derive(sqlx::FromRow)]
pub struct Member {
    /// ID
    pub id: i64,
    /// Username
    pub name: String,
    /// Join Date (in epoch),
    pub joined: i64,
    /// Number of posts
    pub posts: i32,
    /// Title
    pub title: String,
    /// Member Status
    pub mstatus: String,
}

pub async fn get(db: &SqlitePool, id: i32) -> anyhow::Result<Member> {
    let member = sqlx::query_as(
        r#"
SELECT *
FROM ibf_members
WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(member)
}
