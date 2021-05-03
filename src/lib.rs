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

    pub async fn auth(&self, username: &str, password: &str) -> crate::Result<Token> {
        let oauth_clients: OauthClient = self.get("/oauth-clients/local", (), None).await?;
        let params = param::Auth {
            client_id: oauth_clients.client_id,
            client_secret: oauth_clients.client_secret,
            username: username.to_string(),
            password: password.to_string(),
            grant_type: "password".to_string(),
            response_type: "code".to_string(),
        };

        self.post("/users/token", &params, None).await
    }

    async fn get<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(&self, path: &str, params: P, auth: Option<&Token>) -> crate::Result<T> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let client = reqwest::Client::new();
        let mut request = client.get(&url)
            .query(&params);

        if let Some(auth) = auth {
            request = request.bearer_auth(&auth.access_token);
        }

        let response = request.send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::Error::Peertube(response.json().await?))
        }
    }

    async fn post<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(&self, path: &str, params: P, auth: Option<&Token>) -> crate::Result<T> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let client = reqwest::Client::new();
        let mut request = client.post(&url)
            .form(&params);

        if let Some(auth) = auth {
            request = request.bearer_auth(&auth.access_token);
        }

        let response = request.send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::Error::Peertube(response.json().await?))
        }
    }
}

include!("_accounts.rs");

#[cfg(test)]
mod test {
    fn instance() -> String {
        std::env::var("INSTANCE").unwrap()
    }

    fn username() -> String {
        std::env::var("USERNAME").unwrap()
    }

    fn password() -> String {
        std::env::var("PASSWORD").unwrap()
    }

    pub(crate) fn api() -> crate::Api {
        env_logger::try_init().ok();
        dotenv::dotenv().ok();
        crate::Api::new(&instance())
    }

    #[test]
    fn account() {
        let api = crate::test::api();
        let account = tokio_test::block_on(
            api.account(&username())
        ).unwrap();
        assert_eq!(account.display_name, username());
    }

    #[test]
    fn account_videos() {
        let api = crate::test::api();
        let videos = tokio_test::block_on(
            api.account_videos(&username(), &crate::param::Videos::default())
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn accounts() {
        let api = crate::test::api();
        let accounts = tokio_test::block_on(
            api.accounts(&crate::param::Accounts {
                count: Some(2),
                .. Default::default()
            })
        ).unwrap();
        assert_eq!(accounts.data.len(), 2);
    }

    #[test]
    fn account_video_channels() {
        let api = crate::test::api();
        let channels = tokio_test::block_on(
            api.account_video_channels(&username(), &crate::param::Channels::default())
        );
        assert!(channels.is_ok());
    }

    #[test]
    fn auth() {
        let api = crate::test::api();
        let auth = tokio_test::block_on(
            api.auth(
                &username(),
                &password(),
            )
        );
        assert!(auth.is_ok());
    }

    #[test]
    fn accountratings() {
        let api = crate::test::api();
        let auth = tokio_test::block_on(
            api.auth(
                &username(),
                &password(),
            )
        ).unwrap();
        let ratings = tokio_test::block_on(
            api.account_ratings(&auth, &username(), &crate::param::Ratings::default())
        );
        assert!(ratings.is_ok());
    }
}
