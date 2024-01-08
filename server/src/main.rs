use common::{AuthInfo, Counter, Perf, PreAuthInfo};
use rand::{rngs::OsRng, RngCore};
use std::collections::HashMap;
use tokio::sync::RwLock;
use warp::Filter;

use services::auth::AuthResult;
use services::perf::get_perf_code;

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

async fn get_counters(cookie: &str) -> Option<Counter> {
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

    let counters = warp::path!("counters")
        .and(warp::filters::cookie::cookie::<String>("AUTH"))
        .then(|cookie: String| async move { warp::reply::json(&get_counters(&cookie).await) });

    let routes = preauth.or(perf).or(auth).or(counters);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod test {
    use common::{hash_username, PerfToken};

    use super::*;

    #[tokio::test]
    async fn authentication_ok() {
        for i in 0_u32..10 {
            let username = format!("gooduser{}", i);
            let nonce = handle_preauth(PreAuthInfo {
                username: username.clone(),
            })
            .await;
            let token = hash_username(&username, nonce);
            let r = handle_auth(AuthInfo { username, token }).await;
            assert!(r.is_some());
        }
    }

    #[tokio::test]
    async fn authentication_bad() {
        for i in 0_u32..10 {
            let username = format!("baduser{}", i);
            let nonce = handle_preauth(PreAuthInfo {
                username: username.clone(),
            })
            .await;
            let token = hash_username(&username, nonce).overflowing_add(1).0;
            let r = handle_auth(AuthInfo { username, token }).await;
            assert!(r.is_none());
        }
    }

    #[tokio::test]
    async fn perf() {
        let username = "user".to_string();
        let nonce = handle_preauth(PreAuthInfo {
            username: username.clone(),
        })
        .await;
        let token = hash_username(&username, nonce);
        let cookie = handle_auth(AuthInfo { username, token }).await.unwrap();
        let x = store_perf(
            cookie.clone(),
            Perf {
                clicks: PerfToken::from(23),
                pages: PerfToken::from(46),
                speed: PerfToken::from(5645),
            },
        )
        .await;
        assert!(x);
        let c = get_counters(&cookie).await;
        assert_eq!(
            c,
            Some(Counter {
                clicks: 23,
                pages: 46,
                speed: 5645
            })
        )
    }
}
