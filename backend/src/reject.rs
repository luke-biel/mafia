use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    GameInProgress,
    InternalError,
}

impl Reject for Error {}
