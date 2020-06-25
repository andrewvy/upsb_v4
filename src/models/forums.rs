use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

#[derive(sqlx::FromRow)]
pub struct Forum {
    pub id: i64,
    pub topics: i32,
    pub posts: i32,
    pub name: String,
    pub description: String,
}

pub async fn get(db: &SqlitePool, id: i32) -> anyhow::Result<Forum> {
    let forum = sqlx::query_as(
        r#"
SELECT id, topics, posts, name, description
FROM ibf_forums
WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(forum)
}

pub async fn get_all(db: &SqlitePool) -> anyhow::Result<Vec<Forum>> {
    let forums = sqlx::query_as(
        r#"
SELECT id, topics, posts, name, description
FROM ibf_forums
ORDER BY id
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(forums)
}
