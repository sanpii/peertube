pub struct History {
    config: crate::Config,
}

impl History {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List watched videos history.
     */
    pub async fn videos(&self, auth: &crate::data::Token, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: "/users/me/history/videos".into(),
            params: pagination,
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Clear video history.
     */
    pub async fn clear(&self, auth: &crate::data::Token, before_date: &chrono::DateTime<chrono::offset::Utc>) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/me/history/videos/remove".into(),
            params: crate::param::History {
                before_date: *before_date,
            },
            auth: Some(auth.clone()),
        };

        let _: crate::data::Empty = crate::Api::post(&self.config, request).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn videos() {
        let (api, token) = crate::test::api();

        let videos = tokio_test::block_on(
            api.me.history.videos(&token, &crate::param::Pagination::default())
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn clear() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.me.history.clear(&token, &chrono::offset::Utc::now())
        );
        assert!(status.is_ok());
    }
}
