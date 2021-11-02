use backend::cli::handle_admin;
use backend::routes;
use backend::routes::{cors, MAFIA_GUID_COOKIE_NAME};
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
    let events = warp::path("events")
        .and(warp::get())
        .and(warp::cookie::cookie(MAFIA_GUID_COOKIE_NAME))
        .map(|guid| match routes::route_events(guid) {
            Ok(stream) => {
                let stream = stream.map(|msg| warp::sse::Event::default().json_data(msg));
                warp::sse::reply(stream).into_response()
            }
            Err(_) => warp::reply::with_status(warp::reply::json(&()), StatusCode::NOT_FOUND)
                .into_response(),
        });
    let game_state = warp::path("game_state")
        .and(warp::get())
        .and_then(routes::route_game_state);
    let capabilities = warp::path("capabilities")
        .and(warp::post())
        .and(warp::cookie::cookie(MAFIA_GUID_COOKIE_NAME))
        .and(warp::body::json())
        .and_then(routes::route_capabilities);
    let action = warp::path("action")
        .and(warp::post())
        .and(warp::cookie::cookie(MAFIA_GUID_COOKIE_NAME))
        .and(warp::body::json())
        .and_then(routes::route_action);
    let admin = warp::path("admin")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(routes::route_admin);

    tokio::spawn(
        warp::serve(
            register
                .or(user)
                .or(events)
                .or(action)
                .or(game_state)
                .or(capabilities)
                .or(admin)
                .with(cors()),
        )
        .bind(([0, 0, 0, 0], 5069)),
    );

    handle_admin().await
}
