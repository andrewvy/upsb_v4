use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

#[derive(sqlx::FromRow)]
pub struct Thread {
    /// Thread ID
    pub threadid: i32,
    /// Title
    pub title: String,
    /// Number of posts
    pub replycount: i32,
    /// Last post (in epoch)
    pub lastpost: i64,
    /// Thread creator username
    pub postusername: String,
    /// Thread creator user id
    pub postuserid: i32,
    /// Last post username
    pub lastposter: String,
    /// Forum ID
    pub forumid: i32,
}

pub async fn get(db: &SqlitePool, id: i32) -> anyhow::Result<Thread> {
    let thread = sqlx::query_as(
        r#"
SELECT *
FROM thread
WHERE threadid = $1
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(thread)
}

pub async fn get_all(db: &SqlitePool, forum_id: i32) -> anyhow::Result<Vec<Thread>> {
    let threads = sqlx::query_as(
        r#"
SELECT *
FROM thread
WHERE forumid = $1
ORDER BY lastpost DESC
        "#,
    )
    .bind(forum_id)
    .fetch_all(db)
    .await?;

    Ok(threads)
}
