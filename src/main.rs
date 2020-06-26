use std::io::{self, Write};

use dotenv::dotenv;
pub use sqlx::sqlite::SqlitePool;
use warp::http::{Response, StatusCode};
use warp::{path, Filter, Rejection, Reply};

use crate::templates::RenderRucte;

mod bbcode;
mod filters;
mod handlers;
mod models;
mod view;

#[derive(Debug)]
struct SomeError;
impl std::error::Error for SomeError {}
impl warp::reject::Reject for SomeError {}

impl std::fmt::Display for SomeError {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        out.write_str("Some error")
    }
}

/// Create custom error pages.
pub async fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        // We have a custom 404 page!
        Response::builder().status(StatusCode::NOT_FOUND).html(|o| {
            templates::error(
                o,
                StatusCode::NOT_FOUND,
                "The resource you requested could not be located.",
            )
        })
    } else {
        let code = StatusCode::INTERNAL_SERVER_ERROR; // FIXME
        Response::builder()
            .status(code)
            .html(|o| templates::error(o, code, "Something went wrong."))
    }
}

async fn bad_handler() -> Result<StatusCode, Rejection> {
    Err(warp::reject::custom(SomeError))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let pool = SqlitePool::builder()
        .max_size(4)
        .build("sqlite://./upsb_v4.db")
        .await
        .unwrap();

    let routes = warp::get()
        .and(
            path::end()
                .and(filters::with_db(pool.clone()))
                .and_then(filters::home_page)
                .or(filters::forum_filter(pool.clone()))
                .or(filters::member_filter(pool.clone()))
                .or(filters::thread_filter(pool.clone()))
                .or(path("static")
                    .and(path::param())
                    .and_then(filters::static_file))
                .or(path("bad").and_then(bad_handler)),
        )
        .recover(customize_error);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}

/// This method can be used as a "template tag", i.e. a method that
/// can be called directly from a template.
fn footer(out: &mut dyn Write) -> io::Result<()> {
    templates::footer(out)
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
