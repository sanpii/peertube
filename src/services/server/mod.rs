mod redundancy;

pub use redundancy::*;

pub struct Server {
    config: crate::Config,
    pub redundancy: Redundancy,
}

impl Server {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
            redundancy: Redundancy::new(config),
        }
    }
}
