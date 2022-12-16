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
    pub async fn all(
        &self,
        auth: &crate::data::Token,
        params: &crate::param::Redundancies,
    ) -> crate::Result<crate::Pager<crate::data::Redundancy>> {
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
    pub async fn add(&self, auth: &crate::data::Token, video_id: u32) -> crate::Result<()> {
        let params = crate::param::Redundancy { video_id };

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
            path: format!("/server/redundancy/videos/{id}"),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }

    /**
     * Update a server redundancy policy.
     */
    pub async fn update(
        &self,
        auth: &crate::data::Token,
        host: &str,
        allowed: bool,
    ) -> crate::Result<()> {
        let params = crate::param::RedundancySetting {
            redundancy_allowed: allowed,
        };

        let request = crate::Request {
            path: format!("/server/redundancy/{host}"),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn all() {
        let (api, token) = crate::test::api().await;
        let params = crate::param::Redundancies {
            target: crate::param::MirrorDirection::RemoteVideos,
            pagination: Default::default(),
        };

        let redundancies = api.server.redundancy.all(&token, &params).await;

        assert!(redundancies.is_ok());
    }

    #[tokio::test]
    async fn add() {
        let (api, token) = crate::test::api().await;

        let status = api.server.redundancy.add(&token, 42).await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api.server.redundancy.delete(&token, "42").await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn update() {
        let (api, token) = crate::test::api().await;

        let status = api
            .server
            .redundancy
            .update(&token, "example.org", true)
            .await;

        assert!(status.is_ok());
    }
}
