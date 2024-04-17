#![allow(unused)]

use std::{net::SocketAddr};
use serde::Deserialize;

use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use axum::{
    body::Body, extract::{Path, Query, State}, http::uri::Uri, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router, Server
};

pub use self::error::{Error, Result};
pub use self::model::{ModelController};

mod ctx;
mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {

    // Initialize ModelController
    let mc = ModelController::new().await?;

    // Initialize router
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth)); // Middleware that check if the user is authenticated

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis) // merge & prefix every route with /api
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // Initialize server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("=> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("=> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    return Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// All Hello Routes
fn routes_hello() -> Router {
    return Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("=> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("=> {:<12} - handler_hello - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}