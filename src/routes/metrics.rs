use crate::routes::Responce;
use prometheus::{self, Encoder};
use warp::Reply;

pub async fn handler() -> Responce<impl Reply> {
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        error!("could not encode prometheus metrics; error={}", e);
    };
    let res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            error!("prometheus metrics could not be from_utf8'd; error={}", e);
            String::default()
        }
    };
    buffer.clear();
    Ok(res)
}
