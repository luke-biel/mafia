use crate::cli::handle_user_command;
use crate::reject::Error;
use warp::hyper::body::Bytes;
use warp::{Rejection, Reply};

pub async fn route_admin(body: Bytes) -> Result<impl Reply, Rejection> {
    let s = String::from_utf8_lossy(&body);
    handle_user_command(s.trim()).map_err(|err| warp::reject::custom(Error::AdminError(err)))
}
