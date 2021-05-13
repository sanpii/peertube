pub struct Search {
    config: crate::Config,
}

impl Search {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * Search videos.
     */
    pub async fn videos(&self, params: &crate::param::SearchVideos) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: "/search/videos".to_string(),
            params: crate::Params::Query(params),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Search channels.
     */
    pub async fn channels(&self, params: &crate::param::SearchChannels) -> crate::Result<crate::Pager<crate::data::Channel>> {
        let request = crate::Request {
            path: "/search/video-channels".to_string(),
            params: crate::Params::Query(params),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn videos() {
        let (api, _) = crate::test::api();
        let params = crate::param::SearchVideos {
            search: "clément".to_string(),

            .. Default::default()
        };

        let videos = tokio_test::block_on(
            api.search.videos(&params)
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn channels() {
        let (api, _) = crate::test::api();
        let params = crate::param::SearchChannels {
            search: "clément".to_string(),

            .. Default::default()
        };

        let channels = tokio_test::block_on(
            api.search.channels(&params)
        );
        assert!(channels.is_ok());
    }
}
