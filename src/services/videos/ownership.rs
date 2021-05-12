pub struct Ownership {
    config: crate::Config,
}

impl Ownership {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List video ownership changes.
     */
    pub async fn all(&self, auth: &crate::data::Token) -> crate::Result<crate::Pager<crate::data::Ownership>> {
        let request = crate::Request {
            path: "/videos/ownership".to_string(),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Accept ownership change request.
     */
    pub async fn accept(&self, auth: &crate::data::Token, id: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/ownership/{}/accept", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Refuse ownership change request.
     */
    pub async fn refuse(&self, auth: &crate::data::Token, id: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/ownership/{}/refuse", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Request ownership change.
     */
    pub async fn create(&self, auth: &crate::data::Token, video_id: &str, username: &str) -> crate::Result<()> {
        let params = crate::param::Ownership {
            username: username.to_string(),
        };

        let request = crate::Request {
            path: format!("/videos/{}/give-ownership", video_id),
            params: crate::Params::Form(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn all() {
        let (api, token) = crate::test::api();

        let ownerships = tokio_test::block_on(
            api.videos.ownership.all(&token)
        );
        assert!(ownerships.is_ok());
    }

    #[test]
    fn accept() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.ownership.accept(&token, "")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn refuse() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.ownership.refuse(&token, "")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn change() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.ownership.create(&token, "", "username")
        );
        assert!(status.is_ok());
    }
}
