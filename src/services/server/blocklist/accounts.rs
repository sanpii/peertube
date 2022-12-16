pub struct Accounts {
    config: crate::Config,
}

impl Accounts {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List account blocks.
     */
    pub async fn all(
        &self,
        auth: &crate::data::Token,
        pagination: &crate::param::Pagination,
    ) -> crate::Result<crate::Pager<crate::data::BlockedAccount>> {
        let request = crate::Request {
            path: "/server/blocklist/accounts".to_string(),
            params: crate::Params::Query(pagination),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Block a account.
     */
    pub async fn add(&self, auth: &crate::data::Token, handle: &str) -> crate::Result<()> {
        let params = crate::data::AccountName {
            account_name: handle.to_string(),
        };

        let request = crate::Request {
            path: "/server/blocklist/accounts".to_string(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Unblock a account by its handle.
     */
    pub async fn delete(&self, auth: &crate::data::Token, handle: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/server/blocklist/accounts/{handle}"),
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

        let accounts = api
            .server
            .blocklist
            .accounts
            .all(&token, &crate::param::Pagination::default())
            .await;

        assert!(accounts.is_ok());
    }

    #[tokio::test]
    async fn add() {
        let (api, token) = crate::test::api().await;

        let status = api
            .server
            .blocklist
            .accounts
            .add(&token, "test@example.org")
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api
            .server
            .blocklist
            .accounts
            .delete(&token, "test@example.org")
            .await;

        assert!(status.is_ok());
    }
}
