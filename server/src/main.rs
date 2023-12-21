use std::collections::HashMap;

use common::{AuthInfo, Counter, Perf, PreAuthInfo};
use rand::{rngs::OsRng, RngCore};
use services::perf::get_perf_code;
use tokio::sync::RwLock;
use warp::Filter;

use crate::services::auth::AuthResult;

mod services;

lazy_static::lazy_static! {
    /* put some global variables here
     */
}

async fn handle_preauth(info: PreAuthInfo) -> u128 {
    todo!()
}

fn generate_cookie() -> String {
    let mut cookie = [0u8; 16];
    OsRng.fill_bytes(&mut cookie);
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, cookie)
}

async fn handle_auth(info: AuthInfo) -> Option<String> {
    todo!()
}

async fn store_perf(cookie: String, p: Perf) -> bool {
    todo!()
}

#[tokio::main]
async fn main() {
    let preauth = warp::path!("preauth")
        .and(warp::body::json::<PreAuthInfo>())
        .then(|info| async {
            let v = handle_preauth(info).await;
            warp::reply::json(&v)
        });

    let auth = warp::path!("auth")
        .and(warp::body::json::<AuthInfo>())
        .then(|t| async {
            let authresult = handle_auth(t).await;
            let (reply, code, cookie) = match authresult {
                None => ("failed", warp::http::StatusCode::FORBIDDEN, String::new()),
                Some(c) => ("ok", warp::http::StatusCode::OK, format!("AUTH={}", c)),
            };
            warp::reply::with_status(warp::reply::with_header(reply, "Set-Cookie", cookie), code)
        });

    let perf = warp::path!("perf")
        .and(warp::filters::cookie::cookie::<String>("AUTH"))
        .and(warp::body::json::<Perf>())
        .then(|cookie, p| async {
            let success = store_perf(cookie, p).await;
            if success {
                warp::reply::with_status("ok", warp::http::StatusCode::OK)
            } else {
                warp::reply::with_status("failure", warp::http::StatusCode::FORBIDDEN)
            }
        });

    let routes = preauth.or(perf).or(auth);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
