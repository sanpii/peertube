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
    #[test]
    fn all() {
        let (api, _) = crate::test::api();

        let channels = tokio_test::block_on(
            api.channels.all()
        );
        assert!(channels.is_ok());
    }

    #[test]
    fn create() {
        let (api, token) = crate::test::api();
        let params = crate::param::Channel {
            display_name: "New channel".to_string(),
            name: "new-channel".to_string(),

            .. Default::default()
        };

        let status = tokio_test::block_on(
            api.channels.create(&token, &params)
        );
        assert!(status.is_ok());
    }

    #[test]
    fn get() {
        let (api, _) = crate::test::api();

        let channel = tokio_test::block_on(
            api.channels.get("edl@tube.homecomputing.fr")
        );
        assert!(channel.is_ok());
    }

    #[test]
    fn update() {
        let (api, token) = crate::test::api();
        let params = crate::param::ChannelSetting {
            display_name: Some("New channel name".to_string()),

            .. Default::default()
        };

        let status = tokio_test::block_on(
            api.channels.update(&token, "new-channel", &params)
        );
        assert!(status.is_ok());
    }

    #[test]
    fn delete() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.channels.delete(&token, "new-channel")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn videos() {
        let (api, _) = crate::test::api();

        let channel = tokio_test::block_on(
            api.channels.videos("58edd166-dab0-4a1e-86e3-85778b78ba77", &crate::param::Videos::default())
        );
        assert!(channel.is_ok());
    }

    #[test]
    fn update_avatar() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.channels.update_avatar(&token, "58edd166-dab0-4a1e-86e3-85778b78ba77", "fixtures/avatar.png")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn delete_avatar() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.channels.delete_avatar(&token, "58edd166-dab0-4a1e-86e3-85778b78ba77")
        );
        assert!(status.is_ok());
    }
}
