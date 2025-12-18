use crate::db::AppState;
use crate::middleware::auth::auth;
use crate::routes::user_routes::user_routes;
use axum::middleware;
use axum::Router;


pub fn create_app() -> Router<> {
    let state = AppState::new();

    Router::new()
        .nest("/users", user_routes(state.clone()))
        .layer(middleware::from_fn(auth))
        .with_state(state)
}