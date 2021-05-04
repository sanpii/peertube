pub mod data;
pub mod param;
pub mod services;

mod errors;

pub use errors::*;

#[derive(Debug, serde::Deserialize)]
pub struct Pager<T> {
    total: usize,
    data: Vec<T>,
}

#[derive(Clone)]
struct Config {
    base_url: String,
}

pub struct Api {
    config: Config,
    pub accounts: services::Accounts,
}

impl Api {
    pub fn new(base_url: &str) -> Self {
        let config = Config {
            base_url: base_url.to_string(),
        };

        Self {
            accounts: services::Accounts::new(&config),
            config,
        }
    }

    pub async fn auth(&self, username: &str, password: &str) -> crate::Result<data::Token> {
        let oauth_clients: data::OauthClient = Self::get(&self.config, "/oauth-clients/local", (), None).await?;
        let params = param::Auth {
            client_id: oauth_clients.client_id,
            client_secret: oauth_clients.client_secret,
            username: username.to_string(),
            password: password.to_string(),
            grant_type: "password".to_string(),
            response_type: "code".to_string(),
        };

        Self::post(&self.config, "/users/token", &params, None).await
    }

    pub(crate) async fn get<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(config: &Config, path: &str, params: P, auth: Option<&data::Token>) -> crate::Result<T> {
        Self::request(reqwest::Method::GET, config, path, params, auth).await
    }

    pub(crate) async fn post<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(config: &Config, path: &str, params: P, auth: Option<&data::Token>) -> crate::Result<T> {
        Self::request(reqwest::Method::POST, config, path, params, auth).await
    }

    async fn request<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(method: reqwest::Method, config: &Config, path: &str, params: P, auth: Option<&data::Token>) -> crate::Result<T> {
        let url = format!("{}/api/v1{}", config.base_url, path);
        let client = reqwest::Client::new();
        let mut request = client.request(method.clone(), &url);

        request = match method {
            reqwest::Method::GET => request.query(&params),
            _ => request.form(&params),
        };

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
            api.accounts.get(&username())
        ).unwrap();
        assert_eq!(account.display_name, username());
    }

    #[test]
    fn account_videos() {
        let api = crate::test::api();
        let videos = tokio_test::block_on(
            api.accounts.videos(&username(), &crate::param::Videos::default())
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn accounts() {
        let api = crate::test::api();
        let accounts = tokio_test::block_on(
            api.accounts.all(&crate::param::Accounts {
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
            api.accounts.video_channels(&username(), &crate::param::Channels::default())
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
            api.accounts.ratings(&auth, &username(), &crate::param::Ratings::default())
        );
        assert!(ratings.is_ok());
    }
}
