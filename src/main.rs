mod application;
mod config;
mod domain;
mod infrastructure;
mod presentation;
mod utils;

use core::fmt;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::CONFIG;
use tower_http::cors::CorsLayer;

use crate::presentation::router::create_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Couldn't read env");
    // let cors = CorsLayer::new()
    //     .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    //     .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    //     .allow_credentials(true)
    //     .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app = create_router().await.layer(CorsLayer::permissive());

    println!("App created successfully");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &CONFIG.port))
        .await
        .unwrap();
    println!(
        "Serving on http://localhost:{},\ndocs: http://localhost:{}{}",
        &CONFIG.port, &CONFIG.port, &CONFIG.swagger_ui_path,
    );
    axum::serve(listener, app).await.unwrap();
}
