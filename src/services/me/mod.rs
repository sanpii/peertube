mod history;
mod notifications;
mod subscriptions;

pub use history::*;
pub use notifications::*;
pub use subscriptions::*;

pub struct Me {
    config: crate::Config,
    pub history: History,
    pub notifications: Notifications,
    pub subscriptions: Subscriptions,
}

impl Me {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
            history: History::new(config),
            notifications: Notifications::new(config),
            subscriptions: Subscriptions::new(config),
        }
    }

    /**
     * Get my user information.
     */
    pub async fn info(&self, auth: &crate::data::Token) -> crate::Result<crate::data::User> {
        let request = crate::Request {
            path: "/users/me".into(),
            params: (),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Update my user information.
     */
    pub async fn update(&self, auth: &crate::data::Token, params: &crate::param::Me) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/me".into(),
            params,
            auth: Some(auth.clone()),
        };

        let _: crate::data::Empty = crate::Api::put(&self.config, request).await?;

        Ok(())
    }

    /**
     * Get video imports of my user.
     */
    pub async fn imports(&self, auth: &crate::data::Token, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Import>> {
        let request = crate::Request {
            path: "/users/me/videos/imports".into(),
            params: pagination,
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Get my user used quota.
     */
    pub async fn quota(&self, auth: &crate::data::Token) -> crate::Result<crate::data::Quota> {
        let request = crate::Request {
            path: "/users/me/video-quota-used".into(),
            params: (),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Get rate of my user for a video.
     */
    pub async fn video_rating(&self, auth: &crate::data::Token, id: &str) -> crate::Result<crate::data::Rating> {
        let request = crate::Request {
            path: format!("/users/me/videos/{}/rating", id),
            params: (),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Get videos of my user.
     */
    pub async fn videos(&self, auth: &crate::data::Token, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: "/users/me/videos".into(),
            params: pagination,
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Update my user avatar.
     */
    pub async fn update_avatar(&self, auth: &crate::data::Token, avatarfile: &str) -> crate::Result<crate::data::Avatar> {
        let request = crate::Request {
            path: "/users/me/avatar/pick".into(),
            params: crate::param::Avatar {
                avatarfile: avatarfile.to_string(),
            },
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Delete my avatar.
     */
    pub async fn delete_avatar(&self, auth: &crate::data::Token) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/me/avatar".into(),
            params: (),
            auth: Some(auth.clone()),
        };

        let _: crate::data::Empty = crate::Api::post(&self.config, request).await?;

        Ok(())
    }

    /**
     * List my abuses.
     */
    pub async fn abuses(&self, auth: &crate::data::Token, params: &crate::param::Abuse) -> crate::Result<crate::Pager<crate::data::Abuse>> {
        let request = crate::Request {
            path: "/users/me/abuses".into(),
            params,
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
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
