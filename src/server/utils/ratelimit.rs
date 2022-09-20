use std::{net::IpAddr, num::NonZeroU32, time::Duration};

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use governor::{clock::DefaultClock, state::keyed::DashMapStateStore, Quota, RateLimiter};
use once_cell::sync::Lazy;
use tracing::warn;

use crate::server::error::{Error, Result};

type Limiter<T = IpAddr> = RateLimiter<T, DashMapStateStore<T>, DefaultClock>;

static LIMITER_LOGIN: Lazy<Limiter> = Lazy::new(|| {
    let seconds = Duration::from_secs(60);
    let burst = NonZeroU32::new(10).expect("Non-zero login ratelimit burst");
    RateLimiter::keyed(
        Quota::with_period(seconds)
            .expect("Non-zero login ratelimit seconds")
            .allow_burst(burst),
    )
});

pub fn check_limit_login(ip: &IpAddr) -> Result<()> {
    match LIMITER_LOGIN.check_key(ip) {
        Ok(_) => Ok(()),
        Err(_e) => Err(Error::RateLimit),
    }
}

pub struct ClientIp(pub IpAddr);

const X_FORWARDED_FOR: &str = "x-forwarded-for";

#[async_trait]
impl<S> FromRequestParts<S> for ClientIp
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let addr = req
            .headers
            .get(X_FORWARDED_FOR)
            .and_then(|hv| hv.to_str().ok())
            .and_then(|ip| {
                match ip.find(',') {
                    Some(idx) => &ip[..idx],
                    None => ip,
                }
                .parse()
                .map_err(|_| warn!("'{}' header is malformed: {}", X_FORWARDED_FOR, ip))
                .ok()
            })
            .unwrap_or_else(|| "0.0.0.0".parse().unwrap());

        Ok(Self(addr))
    }
}
