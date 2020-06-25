use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

#[derive(sqlx::FromRow)]
pub struct Topic {
    /// Topic ID
    pub tid: i32,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Number of posts
    pub posts: i32,
    /// Last post (in epoch)
    pub last_post: i64,
    /// Pinned
    pub pinned: bool,
    /// State
    pub state: String,
    /// Thread creator user id
    pub starter_id: i32,
    /// Thread creator username
    pub starter_name: String,
    /// Last post user id
    pub last_poster_id: i32,
    /// Last post username
    pub last_poster_name: String,
    /// Forum ID
    pub forum_id: i32,
}

pub async fn get(db: &SqlitePool, id: i32) -> anyhow::Result<Topic> {
    let topic = sqlx::query_as(
        r#"
SELECT *
FROM ibf_topics
WHERE tid = $1
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(topic)
}

pub async fn get_all(db: &SqlitePool, forum_id: i32) -> anyhow::Result<Vec<Topic>> {
    let topics = sqlx::query_as(
        r#"
SELECT *
FROM ibf_topics
WHERE forum_id = $1
ORDER BY last_post DESC
        "#,
    )
    .bind(forum_id)
    .fetch_all(db)
    .await?;

    Ok(topics)
}
