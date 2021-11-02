pub mod action;
pub mod admin;
pub mod capabilities;
pub mod events;
pub mod game_state;
pub mod register;
pub mod user;

pub use action::route_action;
pub use admin::route_admin;
pub use capabilities::route_capabilities;
pub use events::route_events;
pub use game_state::route_game_state;
pub use register::route_register;
pub use user::route_user;
use warp::cors::Cors;

pub const MAFIA_GUID_COOKIE_NAME: &str = "mafia-guid";

pub fn cors() -> Cors {
    warp::cors()
        .allow_any_origin()
        .allow_credentials(true)
        .allow_headers(vec![
            "Referer",
            "Content-Type",
            "Authorization",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Host",
            "Connection",
            "Cache-Control",
            "User-Agent",
            "Accept",
            "Sec-GPC",
            "Origin",
            "Sec-Fetch-Site",
            "Sec-Fetch-Mode",
            "Sec-Fetch-Dest",
            "Referer",
            "Accept-Encoding",
            "Accept-Language",
            "Cookie",
        ])
        .allow_methods(vec!["POST", "GET", "OPTIONS"])
        .build()
}
