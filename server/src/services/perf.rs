/*
simulates a remote storage for performance data
 */

use std::time::Duration;

use common::{Clicks, Pages, PerfToken, Speed};

pub trait TokenDelay {
    fn delay(t: &PerfToken<Self>) -> Duration
    where
        Self: Sized;
}

impl TokenDelay for Clicks {
    fn delay(t: &PerfToken<Self>) -> Duration
    where
        Self: Sized,
    {
        Duration::from_millis((t.token % 25) as u64)
    }
}

impl TokenDelay for Pages {
    fn delay(t: &PerfToken<Self>) -> Duration
    where
        Self: Sized,
    {
        Duration::from_millis((t.token % 45) as u64)
    }
}

impl TokenDelay for Speed {
    fn delay(t: &PerfToken<Self>) -> Duration
    where
        Self: Sized,
    {
        Duration::from_millis((t.token % 10) as u64)
    }
}

pub async fn get_perf_code<A: TokenDelay>(t: PerfToken<A>) -> u64 {
    let d = TokenDelay::delay(&t);
    tokio::time::sleep(d * 10).await;
    t.token as u64
}
