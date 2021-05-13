mod servers;

pub use servers::*;

pub struct Blocklist {
    pub servers: Servers,
}

impl Blocklist {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            servers: Servers::new(config),
        }
    }
}
