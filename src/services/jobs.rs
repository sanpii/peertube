pub struct Jobs {
    config: crate::Config,
}

impl Jobs {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List instance jobs.
     */
    pub async fn all(
        &self,
        auth: &crate::data::Token,
        state: crate::data::JobState,
        pagination: &crate::param::Pagination,
    ) -> crate::Result<crate::Pager<crate::data::Job>> {
        let request = crate::Request {
            path: format!("/jobs/{}", state),
            params: crate::Params::Query(pagination),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn all() {
        let (api, token) = crate::test::api().await;

        let jobs = api
            .jobs
            .all(
                &token,
                crate::data::JobState::All,
                &crate::param::Pagination::default(),
            )
            .await;

        assert!(jobs.is_ok());
    }
}
