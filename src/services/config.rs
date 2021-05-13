pub struct Config {
    config: crate::Config,
}

impl Config {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * Get instance public configuration.
     */
    pub async fn get(&self) -> crate::Result<crate::data::Config> {
        crate::Api::get(&self.config, "/config".into()).await
    }

    /**
     * Get instance "About" information.
     */
    pub async fn about(&self) -> crate::Result<crate::data::config::About> {
        crate::Api::get(&self.config, "/config/about".into()).await
    }

    /**
     * Get instance runtime configuration.
     */
    pub async fn custom(
        &self,
        auth: &crate::data::Token,
    ) -> crate::Result<crate::data::config::Custom> {
        let request = crate::Request {
            path: "/config/custom".to_string(),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Set instance runtime configuration.
     */
    pub async fn set(
        &self,
        _auth: &crate::data::Token,
        _config: crate::data::config::Custom,
    ) -> crate::Result<()> {
        todo!()
    }

    /**
     * Delete instance runtime configuration.
     */
    pub async fn delete(&self, auth: &crate::data::Token) -> crate::Result<()> {
        let request = crate::Request {
            path: "/config/custom".to_string(),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn get() {
        let (api, _) = crate::test::api().await;

        let config = api.config.get().await;

        assert!(config.is_ok());
    }

    #[tokio::test]
    async fn about() {
        let (api, _) = crate::test::api().await;

        let about = api.config.about().await;

        assert!(about.is_ok());
    }

    #[tokio::test]
    async fn custom() {
        let (api, token) = crate::test::api().await;

        let custom = api.config.custom(&token).await;

        assert!(custom.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let custom = api.config.delete(&token).await;

        assert!(custom.is_ok());
    }
}
