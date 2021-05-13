pub struct Plugins {
    config: crate::Config,
}

impl Plugins {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List plugins.
     */
    pub async fn all(
        &self,
        auth: &crate::data::Token,
        pagination: &crate::param::Abuses,
    ) -> crate::Result<crate::Pager<crate::data::Plugin>> {
        let request = crate::Request {
            path: "/plugins".to_string(),
            params: crate::Params::Query(pagination),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List available plugins.
     */
    pub async fn available(
        &self,
        auth: &crate::data::Token,
        pagination: &crate::param::Abuses,
    ) -> crate::Result<crate::Pager<crate::data::AvailablePlugin>> {
        let request = crate::Request {
            path: "/plugins/available".to_string(),
            params: crate::Params::Query(pagination),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Install a plugin.
     */
    pub async fn install(&self, auth: &crate::data::Token, npm_name: &str) -> crate::Result<()> {
        self.action("install", auth, npm_name).await
    }

    /**
     * Update a plugin.
     */
    pub async fn update(&self, auth: &crate::data::Token, npm_name: &str) -> crate::Result<()> {
        self.action("update", auth, npm_name).await
    }

    /**
     * Uninstall a plugin.
     */
    pub async fn uninstall(&self, auth: &crate::data::Token, npm_name: &str) -> crate::Result<()> {
        self.action("uninstall", auth, npm_name).await
    }

    async fn action(
        &self,
        action: &str,
        auth: &crate::data::Token,
        npm_name: &str,
    ) -> crate::Result<()> {
        let params = crate::param::Plugin {
            npm_name: npm_name.to_string(),
        };

        let request = crate::Request {
            path: format!("/plugins/{}", action),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Get a plugin.
     */
    pub async fn get(
        &self,
        auth: &crate::data::Token,
        npm_name: &str,
    ) -> crate::Result<crate::data::Plugin> {
        let request = crate::Request {
            path: format!("/plugins/{}", npm_name),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Set a plugin's settings.
     */
    pub async fn settings(
        &self,
        auth: &crate::data::Token,
        npm_name: &str,
        settings: crate::param::PluginSettings,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/plugins/{}/settings", npm_name),
            params: crate::Params::Json(settings),
            auth: Some(auth.clone()),
        };

        crate::Api::put(&self.config, request).await
    }

    /**
     * Get a plugin's public settings.
     */
    pub async fn public_settings(
        &self,
        npm_name: &str,
    ) -> crate::Result<crate::data::PublicSettings> {
        crate::Api::get(
            &self.config,
            format!("/plugins/{}/public-settings", npm_name).into(),
        )
        .await
    }

    /**
     * Get a plugin's registered settings.
     */
    pub async fn registered_settings(
        &self,
        auth: &crate::data::Token,
        npm_name: &str,
    ) -> crate::Result<crate::data::RegisteredSettings> {
        let request = crate::Request {
            path: format!("/plugins/{}/registered-settings", npm_name),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn all() {
        let (api, token) = crate::test::api().await;

        let plugins = api
            .plugins
            .all(&token, &crate::param::Abuses::default())
            .await;

        assert!(plugins.is_ok());
    }

    #[tokio::test]
    async fn available() {
        let (api, token) = crate::test::api().await;

        let plugins = api
            .plugins
            .available(&token, &crate::param::Abuses::default())
            .await;

        assert!(plugins.is_ok());
    }

    #[tokio::test]
    async fn install() {
        let (api, token) = crate::test::api().await;

        let status = api.plugins.install(&token, "peertube-theme-dark").await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn update() {
        let (api, token) = crate::test::api().await;

        let status = api.plugins.update(&token, "peertube-theme-dark").await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn uninstall() {
        let (api, token) = crate::test::api().await;

        let status = api.plugins.uninstall(&token, "peertube-theme-dark").await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn get() {
        let (api, token) = crate::test::api().await;

        let plugin = api.plugins.get(&token, "peertube-theme-dark").await;

        assert!(plugin.is_ok());
    }

    #[tokio::test]
    async fn settings() {
        let (api, token) = crate::test::api().await;
        let settings = crate::param::PluginSettings {};

        let status = api
            .plugins
            .settings(&token, "peertube-theme-dark", settings)
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn public_settings() {
        let (api, _) = crate::test::api().await;

        let public_settings = api.plugins.public_settings("peertube-theme-dark").await;

        assert!(public_settings.is_ok());
    }

    #[tokio::test]
    async fn registered_settings() {
        let (api, token) = crate::test::api().await;

        let registered_settings = api
            .plugins
            .registered_settings(&token, "peertube-theme-dark")
            .await;

        assert!(registered_settings.is_ok());
    }
}
