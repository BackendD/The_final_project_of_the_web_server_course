use axum ::{Router,routing::{get,post,put,delete}};
use crate::handlers::user_handlers::*;
use crate::db::AppState;

pub fn user_routes(state:AppState)->Router<AppState>{
    Router::new()
        .route("/", get(list_users).post(create_user))
    .route("/:id", get(get_user_by_id).put(update_user).delete(delete_user))
        .with_state(state)
        }