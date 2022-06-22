use actix_governor::{GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};

pub fn get_governor() -> GovernorConfig<PeerIpKeyExtractor> {
    GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap()
}