use crate::routes::Responce;
use warp::{http::StatusCode, Reply};

pub async fn handler() -> Responce<impl Reply> {
    Ok(StatusCode::OK)
}
