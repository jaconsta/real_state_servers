use warp::{Rejection, Error};

pub type Result<T> = std::result::Result<T, Error>;
pub type WebResult<T> = std::result::Result<T, Rejection>;
