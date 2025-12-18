mod app;
pub mod routes;
mod db;
mod handlers;
mod middleware;
mod response;
mod error;
mod models;

use app::create_app;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let app = create_app();
    let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Cannot bind to address");


    println!("Server running at http://127.0.0.1:3000");
    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}