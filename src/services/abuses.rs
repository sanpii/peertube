pub struct Abuses {
    config: crate::Config,
}

impl Abuses {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List abuses.
     */
    pub async fn all(
        &self,
        auth: &crate::data::Token,
        pagination: &crate::param::Abuses,
    ) -> crate::Result<crate::Pager<crate::data::Abuse>> {
        let request = crate::Request {
            path: "/abuses".to_string(),
            params: crate::Params::Query(pagination),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Report an abuse.
     */
    pub async fn report(
        &self,
        auth: &crate::data::Token,
        params: &crate::param::Abuse,
    ) -> crate::Result<crate::data::NewContent> {
        let request = crate::Request {
            path: "/abuses".to_string(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Update an abuse.
     */
    pub async fn update(
        &self,
        auth: &crate::data::Token,
        id: u32,
        params: &crate::param::AbuseSetting,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/abuses/{}", id),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::put(&self.config, request).await
    }

    /**
     * Delete an abuse.
     */
    pub async fn delete(&self, auth: &crate::data::Token, id: u32) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/abuses/{}", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }

    /**
     * List messages of an abuse.
     */
    pub async fn messages(
        &self,
        auth: &crate::data::Token,
        id: u32,
    ) -> crate::Result<crate::Pager<crate::data::AbuseMessage>> {
        let request = crate::Request {
            path: format!("/abuses/{}/messages", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Add message to an abuse.
     */
    pub async fn add_message(
        &self,
        auth: &crate::data::Token,
        id: u32,
        message: &str,
    ) -> crate::Result<crate::data::NewContent> {
        let params = crate::param::AbuseMessage {
            message: message.to_string(),
        };

        let request = crate::Request {
            path: format!("/abuses/{}/messages", id),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Delete an abuse message.
     */
    pub async fn delete_message(
        &self,
        auth: &crate::data::Token,
        id: u32,
        message_id: u32,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/abuses/{}/messages/{}", id, message_id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn all() {
        let (api, token) = crate::test::api().await;

        let abuses = api
            .abuses
            .all(&token, &crate::param::Abuses::default())
            .await;

        assert!(abuses.is_ok());
    }

    #[tokio::test]
    async fn report() {
        let (api, token) = crate::test::api().await;
        let params = crate::param::Abuse {
            reason: "abuse reason".to_string(),
            abuse_element: crate::param::AbuseElement::Account(crate::param::AbusedAccount {
                id: 1,
            }),
            predefined_reasons: vec![crate::data::AbusePredefinedReason::Privacy],
        };

        let abuse = api.abuses.report(&token, &params).await;

        assert!(abuse.is_ok());
    }

    #[tokio::test]
    async fn update() {
        let (api, token) = crate::test::api().await;
        let params = crate::param::AbuseSetting {
            state: Some(crate::data::AbuseState::Rejected),

            ..Default::default()
        };

        let status = api.abuses.update(&token, 1, &params).await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api.abuses.delete(&token, 1).await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn messages() {
        let (api, token) = crate::test::api().await;

        let messages = api.abuses.messages(&token, 1).await;

        assert!(messages.is_ok());
    }

    #[tokio::test]
    async fn add_message() {
        let (api, token) = crate::test::api().await;

        let messages = api.abuses.add_message(&token, 1, "Message").await;

        assert!(messages.is_ok());
    }

    #[tokio::test]
    async fn delete_message() {
        let (api, token) = crate::test::api().await;

        let status = api.abuses.delete_message(&token, 1, 1).await;

        assert!(status.is_ok());
    }
}
