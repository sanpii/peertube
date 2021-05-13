mod accounts;
mod servers;

pub use accounts::*;
pub use servers::*;

pub struct Blocklist {
    pub accounts: Accounts,
    pub servers: Servers,
}

impl Blocklist {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            accounts: Accounts::new(config),
            servers: Servers::new(config),
        }
    }
}
