pub struct Runners {
    config: crate::Config,
}

impl Runners {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List runners
     */
    pub async fn all(
        &self,
        auth: &crate::data::Token,
    ) -> crate::Result<crate::Pager<crate::data::Runner>> {
        let request = crate::Request {
            path: "/runners".to_string(),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Delete a runner.
     */
    pub async fn delete(
        &self,
        auth: &crate::data::Token,
        id: u32,
        runner: &crate::param::Runner,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/runners/{id}"),
            params: crate::Params::Json(runner),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }

    /**
     * Register a Runner
     */
    pub async fn register(
        &self,
        runner: &crate::param::NewRunner,
    ) -> crate::Result<crate::data::NewRunner> {
        let request = crate::Request {
            path: "/runners/register".to_string(),
            params: crate::Params::Json(runner),
            auth: None,
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Unregister a runner
     */
    pub async fn unregister(&self, runner: &crate::param::Runner) -> crate::Result<()> {
        let request = crate::Request {
            path: "/runners/unregister".to_string(),
            params: crate::Params::Json(runner),
            auth: None,
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn all() {
        let (api, token) = crate::test::api().await;

        let runners = api.runners.all(&token).await;

        assert!(runners.is_ok());
    }
}
