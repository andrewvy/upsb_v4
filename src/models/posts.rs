use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

#[derive(sqlx::FromRow)]
pub struct Post {
    /// Post ID
    pub pid: i32,
    /// Author ID
    pub author_id: i32,
    /// Author Username
    pub author_name: String,
    /// Date post was posted (in epoch)
    pub post_date: i64,
    /// Post content
    pub post: String,
}

pub async fn get_all(db: &SqlitePool, topic_id: i32) -> anyhow::Result<Vec<Post>> {
    let posts = sqlx::query_as(
        r#"
SELECT *
FROM ibf_posts
WHERE topic_id = $1
ORDER BY post_date ASC
        "#,
    )
    .bind(topic_id)
    .fetch_all(db)
    .await?;

    Ok(posts)
}
