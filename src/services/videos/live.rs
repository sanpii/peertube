pub struct Live {
    config: crate::Config,
}

impl Live {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * Create a live.
     */
    pub async fn create(&self, auth: &crate::data::Token, params: &crate::param::Live) -> crate::Result<crate::data::NewContent> {
        let request = crate::Request {
            path: "/videos/live".to_string(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Get information about a live.
     */
    pub async fn get(&self, auth: &crate::data::Token, id: &str) -> crate::Result<crate::data::Live> {
        let request = crate::Request {
            path: format!("/videos/live/{}", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Update information about a live.
     */
    pub async fn update(&self, auth: &crate::data::Token, id: &str, params: &crate::param::LiveSetting) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/live/{}", id),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::put::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn create() {
        let (api, token) = crate::test::api();
        let params = crate::param::Live {
            video: crate::param::NewVideo {
                channel_id: "58edd166-dab0-4a1e-86e3-85778b78ba77".to_string(),
                name: "test".to_string(),

                .. Default::default()
            },

            .. Default::default()
        };

        let video = tokio_test::block_on(
            api.videos.live.create(&token, &params)
        );
        assert!(video.is_ok());
    }

    #[test]
    fn get() {
        let (api, token) = crate::test::api();
        let live = tokio_test::block_on(
            api.videos.live.get(&token, "04193a18-7abc-4803-bec7-c75d9888256f")
        );
        assert!(live.is_ok());
    }

    #[test]
    fn update() {
        let (api, token) = crate::test::api();
        let params = crate::param::LiveSetting {
            save_replay: Some(true),
            permanent_live: Some(false),
        };

        let status = tokio_test::block_on(
            api.videos.live.update(&token, "04193a18-7abc-4803-bec7-c75d9888256f", &params)
        );
        assert!(status.is_ok());
    }
}
