use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    GameInProgress,
    InternalError,
    UserNotFound,
    UnsupportedAction,
}

impl Reject for Error {}
