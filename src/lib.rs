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
    pub users: services::Users,
    pub me: services::Me,
}

impl Api {
    pub fn new(base_url: &str) -> Self {
        let config = Config {
            base_url: base_url.to_string(),
        };

        Self {
            accounts: services::Accounts::new(&config),
            users: services::Users::new(&config),
            me: services::Me::new(&config),
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

    pub(crate) async fn delete<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(config: &Config, path: &str, params: P, auth: Option<&data::Token>) -> crate::Result<T> {
        Self::request(reqwest::Method::DELETE, config, path, params, auth).await
    }

    pub(crate) async fn put<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(config: &Config, path: &str, params: P, auth: Option<&data::Token>) -> crate::Result<T> {
        Self::request(reqwest::Method::PUT, config, path, params, auth).await
    }

    async fn request<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(method: reqwest::Method, config: &Config, path: &str, params: P, auth: Option<&data::Token>) -> crate::Result<T> {
        let url = format!("{}/api/v1{}", config.base_url, path);
        let client = reqwest::Client::new();
        let mut request = client.request(method.clone(), &url);

        request = match (method, path) {
            (reqwest::Method::GET, _) => request.query(&params),
            (reqwest::Method::DELETE, _) => request.form(&params),
            (reqwest::Method::POST, "/users/token") => request.form(&params),
            _ => {
                let body = serde_json::to_string(&params)?;

                if body == "null" {
                    request
                } else {
                    request.json(&params)
                }
            },
        };

        if let Some(auth) = auth {
            request = request.bearer_auth(&auth.access_token);
        }

        let response = request.send()
            .await?;

        if response.status().is_success() {
            let data = response.json().await?;

            Ok(data)
        } else {
            let text = response.text().await?;

            Err(crate::Error::Peertube(text))
        }
    }
}

#[cfg(test)]
mod test {
    pub(crate) fn instance() -> String {
        std::env::var("INSTANCE").unwrap()
    }

    pub(crate) fn username() -> String {
        std::env::var("USERNAME").unwrap()
    }

    pub(crate) fn password() -> String {
        std::env::var("PASSWORD").unwrap()
    }

    pub(crate) fn api() -> (crate::Api, crate::data::Token) {
        dotenv::dotenv().ok();
        env_logger::try_init().ok();

        let api = crate::Api::new(&instance());
        let token = tokio_test::block_on(
            api.auth(&username(), &password())
        ).unwrap();

        (api, token)
    }

    #[test]
    fn auth() {
        let (api, _) = crate::test::api();
        let auth = tokio_test::block_on(
            api.auth(
                &crate::test::username(),
                &crate::test::password(),
            )
        );
        assert!(auth.is_ok());
    }
}
