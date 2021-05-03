pub mod param;

mod errors;

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
    fn accounts() {
        let api = crate::test::api();
        let fut = api.accounts(crate::param::Accounts {
            count: Some(2),
            .. Default::default()
        });
        let accounts = tokio_test::block_on(fut).unwrap();
        assert_eq!(accounts.data.len(), 2);
    }
}
