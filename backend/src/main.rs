use backend::routes;
use backend::routes::cors;
use futures::StreamExt;
use uuid::Uuid;
use warp::Filter;

#[tokio::main]
async fn main() {
    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(routes::route_register);
    let user = warp::path!("user" / Uuid)
        .and(warp::get())
        .and_then(routes::route_user);
    let events = warp::path!("events" / Uuid).and(warp::get()).map(|guid| {
        let stream =
            routes::route_events(guid).map(|msg| warp::sse::Event::default().json_data(msg));
        warp::sse::reply(stream)
    });

    warp::serve(register.or(events).or(user).with(cors()))
        .bind(([0, 0, 0, 0], 5069))
        .await;
}
