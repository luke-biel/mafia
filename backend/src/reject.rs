use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    GameInProgress,
    InternalError,
    UserNotFound,
    UnsupportedAction,

    AdminError(&'static str),
}

impl Reject for Error {}
