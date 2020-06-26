use std::time::{Duration, SystemTime};
use warp::http::{Response, StatusCode};
use warp::{Filter, Rejection, Reply};

use crate::models;
use crate::templates::{self, statics::StaticFile, RenderRucte};
use crate::SqlitePool;

/// Home page handler; just render a template with some arguments.
pub async fn home_page(pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let forums = models::forums::get_all(&pool).await.unwrap();

    Response::builder().html(|o| templates::page(o, &forums))
}

/// A duration to add to current time for a far expires header.
static FAR: Duration = Duration::from_secs(180 * 24 * 60 * 60);

/// Handler for static files.
/// Create a response from the file data with a correct content type
/// and a far expires header (or a 404 if the file does not exist).
pub async fn static_file(name: String) -> Result<impl Reply, Rejection> {
    if let Some(data) = StaticFile::get(&name) {
        let _far_expires = SystemTime::now() + FAR;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", data.mime.as_ref())
            // TODO .header("expires", _far_expires)
            .body(data.content))
    } else {
        Err(warp::reject::not_found())
    }
}

pub fn with_db(
    db: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn forum_filter(
    db: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("forum" / i32)
        .and(with_db(db))
        .and_then(forum_handler)
}

async fn forum_handler(forum_id: i32, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let forum = models::forums::get(&pool, forum_id).await.unwrap();
    let threads = models::threads::get_all(&pool, forum_id).await.unwrap();

    Response::builder().html(|o| templates::forum(o, &forum, &threads))
}

pub fn member_filter(
    db: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("member" / i32)
        .and(with_db(db))
        .and_then(member_handler)
}

async fn member_handler(member_id: i32, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let member = models::members::get(&pool, member_id)
        .await
        .map_err(|_| warp::reject::not_found())?;

    Response::builder().html(|o| templates::member(o, &member))
}

pub fn thread_filter(
    db: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("thread" / i32)
        .and(with_db(db))
        .and_then(thread_handler)
}

async fn thread_handler(thread_id: i32, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    let thread = models::threads::get(&pool, thread_id)
        .await
        .map_err(|_| warp::reject::not_found())?;
    let forum = models::forums::get(&pool, thread.forumid).await.unwrap();
    let posts = models::posts::get_all(&pool, thread_id).await.unwrap();

    Response::builder().html(|o| templates::thread(o, &forum, &thread, &posts))
}
