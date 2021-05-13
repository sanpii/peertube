/**
 * <https://docs.joinpeertube.org/api-rest-reference.html#tag/Accounts>
 */
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
     * Get an account.
     */
    pub async fn get(&self, name: &str) -> crate::Result<crate::data::Account> {
        crate::Api::get(&self.config, format!("/accounts/{}", name).into()).await
    }

    /**
     * List videos of an account.
     */
    pub async fn videos(&self, name: &str, params: &crate::param::Videos) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: format!("/accounts/{}/videos", name),
            params: crate::Params::Query(params),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List accounts.
     */
    pub async fn all(&self, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Account>> {
        let request = crate::Request {
            path: "/accounts".to_string(),
            params: crate::Params::Query(pagination),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List video channels of an account.
     */
    pub async fn video_channels(&self, name: &str, params: &crate::param::Channels) -> crate::Result<crate::Pager<crate::data::Channel>> {
        let request = crate::Request {
            path: format!("/accounts/{}/video-channels", name),
            params: crate::Params::Query(params),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List ratings of an account.
     */
    pub async fn ratings(&self, auth: &crate::data::Token, name: &str, params: &crate::param::Ratings) -> crate::Result<crate::Pager<crate::data::Channel>> {
        let request = crate::Request {
            path: format!("/accounts/{}/ratings", name),
            params: crate::Params::Query(params),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn account() {
        let (api, _) = crate::test::api().await;
        let account = api.accounts.get(&crate::test::username())
            .await
            .unwrap();

        assert_eq!(account.display_name, crate::test::username());
    }

    #[tokio::test]
    async fn account_videos() {
        let (api, _) = crate::test::api().await;
        let videos = api.accounts.videos(&crate::test::username(), &crate::param::Videos::default())
            .await;

        assert!(videos.is_ok());
    }

    #[tokio::test]
    async fn accounts() {
        let (api, _) = crate::test::api().await;

        let accounts = api.accounts.all(&crate::param::Pagination {
            count: Some(2),
            .. Default::default()
        }).await.unwrap();

        assert_eq!(accounts.data.len(), 2);
    }

    #[tokio::test]
    async fn account_video_channels() {
        let (api, _) = crate::test::api().await;
        let channels = api.accounts.video_channels(&crate::test::username(), &crate::param::Channels::default())
            .await;

        assert!(channels.is_ok());
    }

    #[tokio::test]
    async fn accountratings() {
        let (api, token) = crate::test::api().await;
        let ratings = api.accounts.ratings(&token, &crate::test::username(), &crate::param::Ratings::default())
            .await;

        assert!(ratings.is_ok());
    }
}
