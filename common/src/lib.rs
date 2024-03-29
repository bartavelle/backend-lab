use std::marker::PhantomData;

use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PreAuthInfo {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthInfo {
    pub username: String,
    pub token: u128,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Perf {
    pub clicks: PerfToken<Clicks>,
    pub pages: PerfToken<Pages>,
    pub speed: PerfToken<Speed>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct Counter {
    pub clicks: u64,
    pub pages: u64,
    pub speed: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct PerfToken<A> {
    pub token: u64,
    #[serde(skip)]
    d: PhantomData<A>,
}

impl<A> PerfToken<A> {
    pub fn new() -> Self {
        Self {
            token: OsRng.next_u64(),
            d: PhantomData {},
        }
    }
}

impl<A> From<u64> for PerfToken<A> {
    fn from(token: u64) -> Self {
        Self {
            token,
            d: PhantomData {},
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Clicks;
#[derive(PartialEq, Eq)]
pub struct Pages;
#[derive(PartialEq, Eq)]
pub struct Speed;

pub fn hash_username(username: &str, nonce: u128) -> u128 {
    username.as_bytes().iter().fold(nonce, |a, b| {
        a.wrapping_mul(18446744073709551557)
            .wrapping_add(*b as u128)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn format() {
        assert_eq!(
            serde_json::to_string(&Perf {
                clicks: PerfToken::from(54),
                pages: PerfToken::from(34),
                speed: PerfToken::from(54),
            })
            .unwrap(),
            "{\"clicks\":54,\"pages\":34,\"speed\":54}"
        )
    }
}
