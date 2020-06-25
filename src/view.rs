use chrono::{TimeZone, Utc};

use crate::templates::Html;

pub fn decode(string: &str) -> String {
    htmlescape::decode_html(string).unwrap_or(string.to_owned())
}

pub fn from_html(string: &str) -> Html<String> {
    Html(decode(string))
}

pub fn epoch_to_datetime(epoch: i64) -> String {
    Utc.timestamp(epoch, 0)
        .format("%a, %b %e %Y %T")
        .to_string()
}
