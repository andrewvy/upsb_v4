use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

#[derive(sqlx::FromRow)]
pub struct Forum {
    pub forumid: i64,
    /// Number of threads
    pub threadcount: i32,
    /// Number of posts
    pub replycount: i32,
    /// Forum title
    pub title: String,
    /// Forum description
    pub description: String,
}

pub async fn get(db: &SqlitePool, id: i32) -> anyhow::Result<Forum> {
    let forum = sqlx::query_as(
        r#"
SELECT *
FROM forum
WHERE forumid = $1
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
SELECT *
FROM forum
ORDER BY displayorder, forumid ASC
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(forums)
}
