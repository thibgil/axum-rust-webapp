use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::{web, Error, Result};


pub fn routes() -> Router {
    return Router::new().route("/api/login", post(api_login));
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>>{
    println!("=> {:<12} - api_login", "HANDLER");
    
    // TODO: Implement a real db/auth logic.
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // Set Cookies when logged in.
    // FIXME: Implement a real auth-token generation/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create the succes body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

