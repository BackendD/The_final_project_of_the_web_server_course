use crate::db::AppState;
use crate::handlers::user_handlers::*;
use axum::{routing::{delete, get, post, put}, Router};

pub fn user_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user_by_id).put(update_user).delete(delete_user))
        .with_state(state)
}