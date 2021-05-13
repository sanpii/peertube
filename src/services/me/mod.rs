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
            params: crate::Params::none(),
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
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::put::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Get video imports of my user.
     */
    pub async fn imports(&self, auth: &crate::data::Token, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Import>> {
        let request = crate::Request {
            path: "/users/me/videos/imports".into(),
            params: crate::Params::Query(pagination),
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
            params: crate::Params::none(),
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
            params: crate::Params::none(),
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
            params: crate::Params::Query(pagination),
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
            params: crate::Params::upload((), "avatarfile", avatarfile)?,
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
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * List my abuses.
     */
    pub async fn abuses(&self, auth: &crate::data::Token, params: &crate::param::Abuse) -> crate::Result<crate::Pager<crate::data::Abuse>> {
        let request = crate::Request {
            path: "/users/me/abuses".into(),
            params: crate::Params::Query(params),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn info() {
        let (api, token) = crate::test::api().await;

        let info = api.me.info(&token)
            .await;

        assert!(info.is_ok());
    }

    #[tokio::test]
    async fn update() {
        let (api, token) = crate::test::api().await;
        let param = crate::param::Me {
            auto_play_video: false,
            display_nsfw: crate::param::DisplayNsfw::True,
            email: "test@example.org".to_string(),
            password: "123456".to_string(),
        };

        let status = api.me.update(&token, &param)
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn imports() {
        let (api, token) = crate::test::api().await;

        let imports = api.me.imports(&token, &crate::param::Pagination::default())
            .await;

        assert!(imports.is_ok());
    }

    #[tokio::test]
    async fn quota() {
        let (api, token) = crate::test::api().await;

        let quota = api.me.quota(&token)
            .await;

        assert!(quota.is_ok());
    }

    #[tokio::test]
    async fn video_rating() {
        let (api, token) = crate::test::api().await;

        let rating = api.me.video_rating(&token, "a83e96ce-0709-4b48-80e3-1462c88d9cc8")
            .await;

        assert!(rating.is_ok());
    }

    #[tokio::test]
    async fn videos() {
        let (api, token) = crate::test::api().await;

        let videos = api.me.videos(&token, &crate::param::Pagination::default())
            .await;

        assert!(videos.is_ok());
    }

    #[tokio::test]
    async fn update_avatar() {
        let (api, token) = crate::test::api().await;

        let status = api.me.update_avatar(&token, "")
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete_avatar() {
        let (api, token) = crate::test::api().await;

        let status = api.me.delete_avatar(&token)
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn abuses() {
        let (api, token) = crate::test::api().await;

        let abuses = api.me.abuses(&token, &crate::param::Abuse::default())
            .await;

        assert!(dbg!(abuses).is_ok());
    }
}
