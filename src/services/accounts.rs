pub struct Accounts {
    config: crate::Config,
}

impl Accounts {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn get(&self, name: &str) -> crate::Result<crate::data::Account> {
        crate::Api::get(&self.config, &format!("/accounts/{}", name), (), None).await
    }

    pub async fn videos(&self, name: &str, params: &crate::param::Videos) -> crate::Result<crate::Pager<crate::data::Video>> {
        crate::Api::get(&self.config, &format!("/accounts/{}/videos", name), params, None).await
    }

    pub async fn all(&self, params: &crate::param::Accounts) -> crate::Result<crate::Pager<crate::data::Account>> {
        crate::Api::get(&self.config, "/accounts", params, None).await
    }

    pub async fn video_channels(&self, name: &str, params: &crate::param::Channels) -> crate::Result<crate::Pager<crate::data::Channel>> {
        crate::Api::get(&self.config, &format!("/accounts/{}/video-channels", name), params, None).await
    }

    pub async fn ratings(&self, auth: &crate::data::Token, name: &str, params: &crate::param::Ratings) -> crate::Result<crate::Pager<crate::data::Channel>> {
        crate::Api::get(&self.config, &format!("/accounts/{}/ratings", name), params, Some(auth)).await
    }
}
