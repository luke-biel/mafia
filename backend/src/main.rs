use backend::routes;
use backend::routes::cors;
use futures::StreamExt;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(routes::route_register);
    let user = warp::path!("user" / Uuid)
        .and(warp::get())
        .and_then(routes::route_user);
    let events =
        warp::path!("events" / Uuid).and(warp::get()).map(|guid| {
            match routes::route_events(guid) {
                Ok(steam) => {
                    let stream = steam.map(|msg| warp::sse::Event::default().json_data(msg));
                    warp::sse::reply(stream).into_response()
                }
                Err(_) => warp::reply::with_status(warp::reply::json(&()), StatusCode::NOT_FOUND)
                    .into_response(),
            }
        });

    warp::serve(register.or(user).or(events).with(cors()))
        .bind(([0, 0, 0, 0], 5069))
        .await;
}
