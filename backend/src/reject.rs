use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    GameInProgress,
    InternalError,
    UserNotFound,
}

impl Reject for Error {}
