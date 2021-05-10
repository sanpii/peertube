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
        crate::Api::get(&self.config, "/users/me/history/videos", pagination, Some(auth)).await
    }

    /**
     * Clear video history.
     */
    pub async fn clear(&self, auth: &crate::data::Token, before_date: &chrono::DateTime<chrono::offset::Utc>) -> crate::Result<()> {
        let param = crate::param::History {
            before_date: *before_date,
        };
        let _: crate::data::Empty = crate::Api::post(&self.config, "/users/me/history/videos/remove", param, Some(auth)).await?;

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
