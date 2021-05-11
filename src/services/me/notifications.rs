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
    pub async fn all(&self, auth: &crate::data::Token, params: &crate::param::Notification) -> crate::Result<crate::Pager<crate::data::Notification>> {
        let request = crate::Request {
            path: "/users/me/notifications".into(),
            params,
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Mark notifications as read by their id.
     */
    pub async fn read(&self, auth: &crate::data::Token, ids: &[u32]) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/me/notifications/read".into(),
            params: crate::param::Notifications {
                ids: ids.to_vec(),
            },
            auth: Some(auth.clone()),
        };

        let _: crate::data::Empty = crate::Api::post(&self.config, request).await?;

        Ok(())
    }

    /**
     * Mark all my notification as read.
     */
    pub async fn read_all(&self, auth: &crate::data::Token) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/me/notifications/read-all".into(),
            params: (),
            auth: Some(auth.clone()),
        };

        let _: crate::data::Empty = crate::Api::post(&self.config, request).await?;

        Ok(())
    }

    /**
     * Update my notification settings.
     */
    pub async fn settings(&self, auth: &crate::data::Token, settings: &crate::param::NotificationSettings) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/me/notification-settings".into(),
            params: settings,
            auth: Some(auth.clone()),
        };

        let _: crate::data::Empty = crate::Api::put(&self.config, request).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn all() {
        let (api, token) = crate::test::api();

        let notifications = tokio_test::block_on(
            api.me.notifications.all(&token, &crate::param::Notification::default())
        );
        assert!(notifications.is_ok());
    }

    #[test]
    fn read() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.me.notifications.read(&token, &[1])
        );
        assert!(status.is_ok());
    }

    #[test]
    fn read_all() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.me.notifications.read_all(&token)
        );
        assert!(status.is_ok());
    }

    #[test]
    fn settings() {
        use crate::param::NotificationSettingsValue::*;

        let (api, token) = crate::test::api();
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

        let status = tokio_test::block_on(
            api.me.notifications.settings(&token, &settings)
        );
        assert!(status.is_ok());
    }
}
