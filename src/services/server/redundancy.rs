pub struct Redundancy {
    config: crate::Config,
}

impl Redundancy {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List videos being mirrored.
     */
    pub async fn all(&self, auth: &crate::data::Token, params: &crate::param::Redundancies) -> crate::Result<crate::Pager<crate::data::Redundancy>> {
        let request = crate::Request {
            path: "/server/redundancy/videos".to_string(),
            params: crate::Params::Query(params),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Mirror a video.
     */
    pub async fn add(&self, auth: &crate::data::Token, video_id: &str) -> crate::Result<()> {
        let params = crate::param::Redundancy {
            video_id: video_id.to_string(),
        };

        let request = crate::Request {
            path: "/server/redundancy/videos".to_string(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Delete a mirror done on a video.
     */
    pub async fn delete(&self, auth: &crate::data::Token, id: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/server/redundancy/videos/{}", id),
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
        let (api, token) = crate::test::api();
        let params = crate::param::Redundancies {
            target: crate::param::MirrorDirection::RemoteVideos,
            pagination: Default::default(),
        };

        let redundancies = tokio_test::block_on(
            api.server.redundancy.all(&token, &params)
        );
        assert!(redundancies.is_ok());
    }

    #[test]
    fn add() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.server.redundancy.add(&token, "42")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn delete() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.server.redundancy.delete(&token, "42")
        );
        assert!(status.is_ok());
    }
}
