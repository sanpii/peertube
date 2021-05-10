mod subscriptions;
mod notifications;

pub use notifications::*;
pub use subscriptions::*;

pub struct Me {
    config: crate::Config,
    pub notifications: Notifications,
    pub subscriptions: Subscriptions,
}

impl Me {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
            notifications: Notifications::new(config),
            subscriptions: Subscriptions::new(config),
        }
    }

    /**
     * Get my user information.
     */
    pub async fn info(&self, auth: &crate::data::Token) -> crate::Result<crate::data::User> {
        crate::Api::get(&self.config, "/users/me", (), Some(auth)).await
    }

    /**
     * Update my user information.
     */
    pub async fn update(&self, auth: &crate::data::Token, params: &crate::param::Me) -> crate::Result<()> {
        let _: crate::data::Empty = crate::Api::put(&self.config, "/users/me", params, Some(auth)).await?;

        Ok(())
    }

    /**
     * Get video imports of my user.
     */
    pub async fn imports(&self, auth: &crate::data::Token, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Import>> {
        crate::Api::get(&self.config, "/users/me/videos/imports", pagination, Some(auth)).await
    }

    /**
     * Get my user used quota.
     */
    pub async fn quota(&self, auth: &crate::data::Token) -> crate::Result<crate::data::Quota> {
        crate::Api::get(&self.config, "/users/me/video-quota-used", (), Some(auth)).await
    }

    /**
     * Get rate of my user for a video.
     */
    pub async fn video_rating(&self, auth: &crate::data::Token, id: &str) -> crate::Result<crate::data::Rating> {
        crate::Api::get(&self.config, &format!("/users/me/videos/{}/rating", id), (), Some(auth)).await
    }

    /**
     * Get videos of my user.
     */
    pub async fn videos(&self, auth: &crate::data::Token, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Video>> {
        crate::Api::get(&self.config, "/users/me/videos", pagination, Some(auth)).await
    }

    /**
     * Update my user avatar.
     */
    pub async fn update_avatar(&self, auth: &crate::data::Token, avatarfile: &str) -> crate::Result<crate::data::Avatar> {
        let avatar = crate::param::Avatar {
            avatarfile: avatarfile.to_string(),
        };
        crate::Api::post(&self.config, "/users/me/avatar/pick", avatar, Some(auth)).await
    }

    /**
     * Delete my avatar.
     */
    pub async fn delete_avatar(&self, auth: &crate::data::Token) -> crate::Result<()> {
        let _: crate::data::Empty = crate::Api::post(&self.config, "/users/me/avatar", (), Some(auth)).await?;

        Ok(())
    }

    /**
     * List my abuses.
     */
    pub async fn abuses(&self, auth: &crate::data::Token, params: &crate::param::Abuse) -> crate::Result<crate::Pager<crate::data::Abuse>> {
        crate::Api::get(&self.config, "/users/me/abuses", params, Some(auth)).await
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn info() {
        let (api, token) = crate::test::api();

        let info = tokio_test::block_on(
            api.me.info(&token)
        );
        assert!(info.is_ok());
    }

    #[test]
    fn update() {
        let (api, token) = crate::test::api();
        let param = crate::param::Me {
            auto_play_video: false,
            display_nsfw: crate::param::DisplayNsfw::True,
            email: "test@example.org".to_string(),
            password: "123456".to_string(),
        };

        let status = tokio_test::block_on(
            api.me.update(&token, &param)
        );
        assert!(status.is_ok());
    }

    #[test]
    fn imports() {
        let (api, token) = crate::test::api();

        let imports = tokio_test::block_on(
            api.me.imports(&token, &crate::param::Pagination::default())
        );
        assert!(imports.is_ok());
    }

    #[test]
    fn quota() {
        let (api, token) = crate::test::api();

        let quota = tokio_test::block_on(
            api.me.quota(&token)
        );
        assert!(quota.is_ok());
    }

    #[test]
    fn video_rating() {
        let (api, token) = crate::test::api();

        let rating = tokio_test::block_on(
            api.me.video_rating(&token, "a83e96ce-0709-4b48-80e3-1462c88d9cc8")
        );
        assert!(rating.is_ok());
    }

    #[test]
    fn videos() {
        let (api, token) = crate::test::api();

        let videos = tokio_test::block_on(
            api.me.videos(&token, &crate::param::Pagination::default())
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn update_avatar() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.me.update_avatar(&token, "")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn delete_avatar() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.me.delete_avatar(&token)
        );
        assert!(status.is_ok());
    }

    #[test]
    fn abuses() {
        let (api, token) = crate::test::api();

        let abuses = tokio_test::block_on(
            api.me.abuses(&token, &crate::param::Abuse::default())
        );
        assert!(dbg!(abuses).is_ok());
    }
}
