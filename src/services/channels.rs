pub struct Channels {
    config: crate::Config,
}

impl Channels {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List video channels.
     */
    pub async fn all(&self) -> crate::Result<crate::Pager<crate::data::Channel>> {
        crate::Api::get(&self.config, "/video-channels".into()).await
    }

    /**
     * Create a video channel.
     */
    pub async fn create(&self, auth: &crate::data::Token, params: &crate::param::Channel) -> crate::Result<crate::data::NewContent> {
        let request = crate::Request {
            path: "/video-channels".to_string(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Get a video channel.
     */
    pub async fn get(&self, handle: &str) -> crate::Result<crate::data::Channel> {
        crate::Api::get(&self.config, format!("/video-channels/{}", handle).into()).await
    }

    /**
     * Update a video channel.
     */
    pub async fn update(&self, auth: &crate::data::Token, handle: &str, params: &crate::param::ChannelSetting) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-channels/{}", handle),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::put::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Delete a video channel.
     */
    pub async fn delete(&self, auth: &crate::data::Token, handle: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-channels/{}", handle),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * List videos of a video channel.
     */
    pub async fn videos(&self, handle: &str, params: &crate::param::Videos) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: format!("/video-channels/{}/videos", handle),
            params: crate::Params::Query(params),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Update channel avatar.
     */
    pub async fn update_avatar(&self, auth: &crate::data::Token, handle: &str, avatarfile: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-channels/{}/avatar/pick", handle),
            params: crate::Params::upload((), "avatarfile", avatarfile)?,
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Delete channel avatar.
     */
    pub async fn delete_avatar(&self, auth: &crate::data::Token, handle: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-channels/{}/avatar", handle),
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
    async fn all() {
        let (api, _) = crate::test::api().await;

        let channels = api.channels.all()
            .await;

        assert!(channels.is_ok());
    }

    #[tokio::test]
    async fn create() {
        let (api, token) = crate::test::api().await;
        let params = crate::param::Channel {
            display_name: "New channel".to_string(),
            name: "new-channel".to_string(),

            .. Default::default()
        };

        let status = api.channels.create(&token, &params)
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn get() {
        let (api, _) = crate::test::api().await;

        let channel = api.channels.get("edl@tube.homecomputing.fr")
            .await;

        assert!(channel.is_ok());
    }

    #[tokio::test]
    async fn update() {
        let (api, token) = crate::test::api().await;
        let params = crate::param::ChannelSetting {
            display_name: Some("New channel name".to_string()),

            .. Default::default()
        };

        let status = api.channels.update(&token, "new-channel", &params)
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api.channels.delete(&token, "new-channel")
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn videos() {
        let (api, _) = crate::test::api().await;

        let channel = api.channels.videos("58edd166-dab0-4a1e-86e3-85778b78ba77", &crate::param::Videos::default())
            .await;

        assert!(channel.is_ok());
    }

    #[tokio::test]
    async fn update_avatar() {
        let (api, token) = crate::test::api().await;

        let status = api.channels.update_avatar(&token, "58edd166-dab0-4a1e-86e3-85778b78ba77", "fixtures/avatar.png")
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete_avatar() {
        let (api, token) = crate::test::api().await;

        let status = api.channels.delete_avatar(&token, "58edd166-dab0-4a1e-86e3-85778b78ba77")
            .await;

        assert!(status.is_ok());
    }
}
