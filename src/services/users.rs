pub struct Users {
    config: crate::Config,
}

impl Users {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * Create a user.
     */
    pub async fn create(
        &self,
        auth: &crate::data::Token,
        user: &crate::param::NewUser,
    ) -> crate::Result<crate::data::NewUser> {
        let request = crate::Request {
            path: "/users".to_string(),
            params: crate::Params::Json(user),
            auth: Some(auth.clone()),
        };

        let data = crate::Api::post(&self.config, request).await?;

        let crate::data::Data::NewUser(user) = data;

        Ok(user)
    }

    /**
     * List users.
     */
    pub async fn all(
        &self,
        auth: &crate::data::Token,
        params: &crate::param::Users,
    ) -> crate::Result<crate::Pager<crate::data::User>> {
        let request = crate::Request {
            path: "/users".to_string(),
            params: crate::Params::Query(params),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Delete a user.
     */
    pub async fn delete(&self, auth: &crate::data::Token, id: u32) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/users/{}", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }

    /**
     * Get a user.
     */
    pub async fn get(
        &self,
        auth: &crate::data::Token,
        id: u32,
    ) -> crate::Result<crate::data::User> {
        let request = crate::Request {
            path: format!("/users/{}", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Update a user.
     */
    pub async fn update(
        &self,
        auth: &crate::data::Token,
        id: u32,
        params: &crate::param::User,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/users/{}", id),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::put(&self.config, request).await
    }

    /**
     * Register a user.
     */
    pub async fn register(&self, params: &crate::param::Register) -> crate::Result<()> {
        let request = crate::Request {
            path: "/users/register".to_string(),
            params: crate::Params::Json(params),
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
    async fn create() {
        let (api, token) = crate::test::api().await;
        let param = crate::param::NewUser {
            email: "test@example.org".to_string(),
            password: "123456".to_string(),
            username: "test".to_string(),

            ..Default::default()
        };
        let user = api.users.create(&token, &param).await;

        assert!(user.is_ok());
    }

    #[tokio::test]
    async fn all() {
        let (api, token) = crate::test::api().await;

        let users = api.users.all(&token, &crate::param::Users::default()).await;

        assert!(users.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api.users.delete(&token, 999).await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn get() {
        let (api, token) = crate::test::api().await;

        let user = api.users.get(&token, 1).await;

        assert!(user.is_ok());
    }

    #[tokio::test]
    async fn update() {
        let (api, token) = crate::test::api().await;
        let param = crate::param::User {
            id: 1,
            video_quota_daily: Some(100),

            ..Default::default()
        };
        let user = api.users.update(&token, 1, &param).await;

        assert!(user.is_ok());
    }

    #[tokio::test]
    async fn register() {
        let (api, _) = crate::test::api().await;
        let param = crate::param::Register {
            email: "test@example.org".to_string(),
            password: "123456".to_string(),
            username: "test".to_string(),

            ..Default::default()
        };
        let user = api.users.register(&param).await;

        assert!(user.is_ok());
    }
}
