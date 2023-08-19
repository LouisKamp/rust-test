use std::collections::HashMap;

use axum::{extract::Query, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello_world))
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world(Query(params): Query<HashMap<String, String>>) -> &'static str {
    tracing::info!("{:?}", params);

    return "Hello, World!";
}
