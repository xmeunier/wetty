use warp::Rejection;

pub mod health;
pub mod html;
pub mod metrics;
pub mod socket;

type Responce<T> = std::result::Result<T, Rejection>;
