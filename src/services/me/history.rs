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
    pub async fn videos(
        &self,
        auth: &crate::data::Token,
        pagination: &crate::param::Pagination,
    ) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: "/users/me/history/videos".into(),
            params: crate::Params::Query(pagination),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Clear video history.
     */
    pub async fn clear(
        &self,
        auth: &crate::data::Token,
        before_date: &chrono::DateTime<chrono::offset::Utc>,
    ) -> crate::Result<()> {
        let params = crate::param::History {
            before_date: *before_date,
        };

        let request = crate::Request {
            path: "/users/me/history/videos/remove".into(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn videos() {
        let (api, token) = crate::test::api().await;

        let videos = api
            .me
            .history
            .videos(&token, &crate::param::Pagination::default())
            .await;

        assert!(videos.is_ok());
    }

    #[tokio::test]
    async fn clear() {
        let (api, token) = crate::test::api().await;

        let status = api
            .me
            .history
            .clear(&token, &chrono::offset::Utc::now())
            .await;

        assert!(status.is_ok());
    }
}
