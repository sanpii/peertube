mod blocklist;
mod redundancy;

pub use blocklist::Blocklist;
pub use redundancy::*;

pub struct Server {
    pub blocklist: Blocklist,
    pub redundancy: Redundancy,
}

impl Server {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            blocklist: Blocklist::new(config),
            redundancy: Redundancy::new(config),
        }
    }
}
