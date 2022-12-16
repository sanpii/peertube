pub struct Servers {
    config: crate::Config,
}

impl Servers {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List server blocks.
     */
    pub async fn all(
        &self,
        auth: &crate::data::Token,
        pagination: &crate::param::Pagination,
    ) -> crate::Result<crate::Pager<crate::data::BlockedServer>> {
        let request = crate::Request {
            path: "/server/blocklist/servers".to_string(),
            params: crate::Params::Query(pagination),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Block a server.
     */
    pub async fn add(&self, auth: &crate::data::Token, host: &str) -> crate::Result<()> {
        let params = crate::data::Server {
            host: host.to_string(),
        };

        let request = crate::Request {
            path: "/server/blocklist/servers".to_string(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Unblock a server by its domain.
     */
    pub async fn delete(&self, auth: &crate::data::Token, host: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/server/blocklist/servers/{host}"),
            params: crate::Params::none(),
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

        let servers = api
            .server
            .blocklist
            .servers
            .all(&token, &crate::param::Pagination::default())
            .await;

        assert!(servers.is_ok());
    }

    #[tokio::test]
    async fn add() {
        let (api, token) = crate::test::api().await;

        let status = api
            .server
            .blocklist
            .servers
            .add(&token, "example.org")
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api
            .server
            .blocklist
            .servers
            .delete(&token, "example.org")
            .await;

        assert!(status.is_ok());
    }
}
