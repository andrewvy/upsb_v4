use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

#[derive(sqlx::FromRow)]
pub struct Post {
    /// Post ID
    pub postid: i32,
    /// Author ID
    pub userid: i32,
    /// Author Username
    pub username: String,
    /// Date post was posted (in epoch)
    pub dateline: i64,
    /// Post content
    pub pagetext: String,
}

pub async fn get_all(db: &SqlitePool, thread_id: i32) -> anyhow::Result<Vec<Post>> {
    let posts = sqlx::query_as(
        r#"
SELECT *
FROM post
WHERE threadid = $1
ORDER BY dateline ASC
        "#,
    )
    .bind(thread_id)
    .fetch_all(db)
    .await?;

    Ok(posts)
}
