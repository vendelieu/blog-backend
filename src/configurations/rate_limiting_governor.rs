use actix_governor::{GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_governor::governor::middleware::NoOpMiddleware;

pub fn get_governor() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware> {
    GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .unwrap()
}