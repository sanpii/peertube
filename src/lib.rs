#![warn(rust_2018_idioms)]

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

struct Request<S: serde::Serialize> {
    path: String,
    params: Params<S>,
    auth: Option<data::Token>,
}

enum Params<S: serde::Serialize> {
    Query(S),
    Json(S),
    Form(S),
    Multipart(S, reqwest::multipart::Form),
    None,
}

impl Params<()> {
    fn none() -> Self {
        Params::None
    }
}

impl<S: serde::Serialize> Params<S> {
    fn multipart(params: S) -> Result<Self> {
        let form = reqwest::multipart::Form::new();

        Ok(Self::Multipart(params, form))
    }

    fn upload(params: S, name: &str, file: &str) -> Result<Self> {
        let part = reqwest::multipart::Part::bytes(std::fs::read(file)?)
            .file_name(file.to_string());

        let form = reqwest::multipart::Form::new()
            .part(name.to_string(), part);

        Ok(Self::Multipart(params, form))
    }
}

impl<S: serde::Serialize> Request<S> {
    fn into_request(self, method: reqwest::Method, base_url: &str) -> crate::Result<reqwest::RequestBuilder> {
        let url = format!("{}/api/v1{}", base_url, self.path);
        let client = reqwest::Client::new();
        let mut request = client.request(method, &url);

        request = match self.params {
            Params::Query(params) => request.query(&params),
            Params::Json(params) => request.json(&params),
            Params::Form(params) => request.form(&params),
            Params::Multipart(params, mut form) => {
                match serde_json::to_value(&params)? {
                    serde_json::Value::Object(map) => for (k, v) in map.iter() {
                        form = form.text(k.to_string(), v.to_string());
                    }
                    serde_json::Value::Null => (),
                    _ => unimplemented!(),
                };

                request.multipart(form)
            }
            Params::None => request,
        };

        if let Some(auth) = self.auth {
            request = request.bearer_auth(&auth.access_token);
        }

        Ok(request)
    }
}

impl From<&str> for Request<()> {
    fn from(path: &str) -> Self {
        path.to_string().into()
    }
}

impl From<String> for Request<()> {
    fn from(path: String) -> Self {
        Self {
            path,
            params: Params::None,
            auth: None,
        }
    }
}

pub struct Api {
    config: Config,
    pub accounts: services::Accounts,
    pub channels: services::Channels,
    pub me: services::Me,
    pub playlists: services::Playlists,
    pub search: services::Search,
    pub server: services::Server,
    pub users: services::Users,
    pub videos: services::Videos,
}

impl Api {
    pub fn new(base_url: &str) -> Self {
        let config = Config {
            base_url: base_url.to_string(),
        };

        Self {
            accounts: services::Accounts::new(&config),
            channels: services::Channels::new(&config),
            me: services::Me::new(&config),
            playlists: services::Playlists::new(&config),
            search: services::Search::new(&config),
            server: services::Server::new(&config),
            users: services::Users::new(&config),
            videos: services::Videos::new(&config),
            config,
        }
    }

    pub async fn auth(&self, username: &str, password: &str) -> crate::Result<data::Token> {
        let oauth_clients: data::OauthClient = Self::get(&self.config, "/oauth-clients/local".into()).await?;
        let params = param::Auth {
            client_id: oauth_clients.client_id,
            client_secret: oauth_clients.client_secret,
            username: username.to_string(),
            password: password.to_string(),
            grant_type: "password".to_string(),
            response_type: "code".to_string(),
        };

        let request = Request {
            path: "/users/token".to_string(),
            params: Params::Form(params),
            auth: None,
        };
        Self::post(&self.config, request).await
    }

    pub(crate) async fn get<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(config: &Config, request: Request<P>) -> crate::Result<T> {
        Self::request(reqwest::Method::GET, config, request).await
    }

    pub(crate) async fn post<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(config: &Config, request: Request<P>) -> crate::Result<T> {
        Self::request(reqwest::Method::POST, config, request).await
    }

    pub(crate) async fn delete<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(config: &Config, request: Request<P>) -> crate::Result<T> {
        Self::request(reqwest::Method::DELETE, config, request).await
    }

    pub(crate) async fn put<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(config: &Config, request: Request<P>) -> crate::Result<T> {
        Self::request(reqwest::Method::PUT, config, request).await
    }

    async fn request<T: for<'de> serde::Deserialize<'de>, P: serde::Serialize>(method: reqwest::Method, config: &Config, request: Request<P>) -> crate::Result<T> {
        let response = request.into_request(method, &config.base_url)?
            .send()
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
