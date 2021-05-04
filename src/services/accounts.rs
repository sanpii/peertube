pub struct Accounts {
    config: crate::Config,
}

impl Accounts {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn get(&self, name: &str) -> crate::Result<crate::data::Account> {
        crate::Api::get(&self.config, &format!("/accounts/{}", name), (), None).await
    }

    pub async fn videos(&self, name: &str, params: &crate::param::Videos) -> crate::Result<crate::Pager<crate::data::Video>> {
        crate::Api::get(&self.config, &format!("/accounts/{}/videos", name), params, None).await
    }

    pub async fn all(&self, params: &crate::param::Accounts) -> crate::Result<crate::Pager<crate::data::Account>> {
        crate::Api::get(&self.config, "/accounts", params, None).await
    }

    pub async fn video_channels(&self, name: &str, params: &crate::param::Channels) -> crate::Result<crate::Pager<crate::data::Channel>> {
        crate::Api::get(&self.config, &format!("/accounts/{}/video-channels", name), params, None).await
    }

    pub async fn ratings(&self, auth: &crate::data::Token, name: &str, params: &crate::param::Ratings) -> crate::Result<crate::Pager<crate::data::Channel>> {
        crate::Api::get(&self.config, &format!("/accounts/{}/ratings", name), params, Some(auth)).await
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn account() {
        let api = crate::test::api();
        let account = tokio_test::block_on(
            api.accounts.get(&crate::test::username())
        ).unwrap();
        assert_eq!(account.display_name, crate::test::username());
    }

    #[test]
    fn account_videos() {
        let api = crate::test::api();
        let videos = tokio_test::block_on(
            api.accounts.videos(&crate::test::username(), &crate::param::Videos::default())
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn accounts() {
        let api = crate::test::api();
        let accounts = tokio_test::block_on(
            api.accounts.all(&crate::param::Accounts {
                count: Some(2),
                .. Default::default()
            })
        ).unwrap();
        assert_eq!(accounts.data.len(), 2);
    }

    #[test]
    fn account_video_channels() {
        let api = crate::test::api();
        let channels = tokio_test::block_on(
            api.accounts.video_channels(&crate::test::username(), &crate::param::Channels::default())
        );
        assert!(channels.is_ok());
    }

    #[test]
    fn auth() {
        let api = crate::test::api();
        let auth = tokio_test::block_on(
            api.auth(
                &crate::test::username(),
                &crate::test::password(),
            )
        );
        assert!(auth.is_ok());
    }

    #[test]
    fn accountratings() {
        let api = crate::test::api();
        let auth = tokio_test::block_on(
            api.auth(
                &crate::test::username(),
                &crate::test::password(),
            )
        ).unwrap();
        let ratings = tokio_test::block_on(
            api.accounts.ratings(&auth, &crate::test::username(), &crate::param::Ratings::default())
        );
        assert!(ratings.is_ok());
    }
}
