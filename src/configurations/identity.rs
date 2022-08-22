use actix_identity::{CookieIdentityPolicy, IdentityService};
use crate::consts;

pub fn get_config(domain: String) -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(&consts::SECRET_KEY)
            .name("auth")
            .path("/")
            .domain(domain.as_str())
            .max_age_secs(chrono::Duration::days(1).num_seconds())
            .secure(true), // this can only be true if you have https
    )
}