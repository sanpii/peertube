pub mod param;

mod entity;
mod errors;

use entity::*;
use errors::*;

#[derive(Debug, serde::Deserialize)]
pub struct Pager<T> {
    total: usize,
    data: Vec<T>,
}

pub struct Api {
    base_url: String,
}

impl Api {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get<T: for<'de> serde::Deserialize<'de>, P: ToString>(&self, path: &str, params: &P) -> crate::Result<T> {
        let url = format!("{}/api/v1{}?{}", self.base_url, path, params.to_string());
        let response = reqwest::get(&url)
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

include!("_accounts.rs");

#[cfg(test)]
mod test {
    pub(crate) fn api() -> crate::Api {
        env_logger::try_init().ok();
        crate::Api::new("https://peertube.cpy.re")
    }

    #[test]
    fn account() {
        let api = crate::test::api();
        let account = tokio_test::block_on(
            api.account("chocobozzz")
        ).unwrap();
        assert_eq!(account.display_name, "chocobozzz");
    }

    #[test]
    fn account_videos() {
        let api = crate::test::api();
        let videos = tokio_test::block_on(
            api.account_videos("chocobozzz")
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn accounts() {
        let api = crate::test::api();
        let accounts = tokio_test::block_on(
            api.accounts(crate::param::Accounts {
                count: Some(2),
                .. Default::default()
            })
        ).unwrap();
        assert_eq!(accounts.data.len(), 2);
    }
}
