mod blocklist;
mod redundancy;

pub use blocklist::Blocklist;
pub use redundancy::Redundancy;

pub struct Server {
    config: crate::Config,
    pub blocklist: Blocklist,
    pub redundancy: Redundancy,
}

impl Server {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
            blocklist: Blocklist::new(config),
            redundancy: Redundancy::new(config),
        }
    }

    /**
     * Unfollow a server.
     */
    pub async fn unfollow(&self, auth: &crate::data::Token, host: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/server/following/{}", host),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }

    /**
     * List instances followed by the server.
     */
    pub async fn followings(
        &self,
        pagination: &crate::param::Followings,
    ) -> crate::Result<crate::Pager<crate::data::Follow>> {
        let request = crate::Request {
            path: "/server/following".to_string(),
            params: crate::Params::Query(pagination),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List instance followers.
     */
    pub async fn followers(
        &self,
        pagination: &crate::param::Pagination,
    ) -> crate::Result<crate::Pager<crate::data::Follow>> {
        let request = crate::Request {
            path: "/server/followers".to_string(),
            params: crate::Params::Query(pagination),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Follow a server.
     */
    pub async fn follow(&self, auth: &crate::data::Token, host: &str) -> crate::Result<()> {
        let params = crate::param::Following {
            host: host.to_string(),
        };

        let request = crate::Request {
            path: "/server/following".to_string(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn unfollow() {
        let (api, token) = crate::test::api().await;

        let status = api.server.unfollow(&token, "example.org").await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn followings() {
        let (api, _) = crate::test::api().await;

        let followings = api
            .server
            .followings(&crate::param::Followings::default())
            .await;

        assert!(followings.is_ok());
    }

    #[tokio::test]
    async fn followers() {
        let (api, _) = crate::test::api().await;

        let followers = api
            .server
            .followers(&crate::param::Pagination::default())
            .await;

        assert!(followers.is_ok());
    }

    #[tokio::test]
    async fn follow() {
        let (api, token) = crate::test::api().await;

        let status = api.server.follow(&token, "example.org").await;

        assert!(status.is_ok());
    }
}
