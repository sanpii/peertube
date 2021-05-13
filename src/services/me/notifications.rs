pub struct Notifications {
    config: crate::Config,
}

impl Notifications {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List my notifications.
     */
    pub async fn all(
        &self,
        auth: &crate::data::Token,
        params: &crate::param::Notification,
    ) -> crate::Result<crate::Pager<crate::data::Notification>> {
        let request = crate::Request {
            path: "/users/me/notifications".into(),
            params: crate::Params::Query(params),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Mark notifications as read by their id.
     */
    pub async fn read(&self, auth: &crate::data::Token, ids: &[u32]) -> crate::Result<()> {
        let params = crate::param::Notifications { ids: ids.to_vec() };

        let request = crate::Request {
            path: "/users/me/notifications/read".into(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Mark all my notification as read.
     */
    pub async fn read_all(&self, auth: &crate::data::Token) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/me/notifications/read-all".into(),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Update my notification settings.
     */
    pub async fn settings(
        &self,
        auth: &crate::data::Token,
        settings: &crate::param::NotificationSettings,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/me/notification-settings".into(),
            params: crate::Params::Json(settings),
            auth: Some(auth.clone()),
        };

        crate::Api::put(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn all() {
        let (api, token) = crate::test::api().await;

        let notifications = api
            .me
            .notifications
            .all(&token, &crate::param::Notification::default())
            .await;

        assert!(notifications.is_ok());
    }

    #[tokio::test]
    async fn read() {
        let (api, token) = crate::test::api().await;

        let status = api.me.notifications.read(&token, &[1]).await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn read_all() {
        let (api, token) = crate::test::api().await;

        let status = api.me.notifications.read_all(&token).await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn settings() {
        use crate::param::NotificationSettingsValue::*;

        let (api, token) = crate::test::api().await;
        let settings = crate::param::NotificationSettings {
            abuse_as_moderator: Both,
            auto_instance_following: Web,
            blacklist_on_my_video: Both,
            comment_mention: Web,
            my_video_import_finished: Web,
            my_video_published: Web,
            new_comment_on_my_video: Web,
            new_follow: Web,
            new_instance_follower: Web,
            new_user_registration: Web,
            new_video_from_subscription: Web,
            video_auto_blacklist_as_moderator: Both,
        };

        let status = api.me.notifications.settings(&token, &settings).await;

        assert!(status.is_ok());
    }
}
