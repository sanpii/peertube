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
    pub async fn create(&self, auth: &crate::data::Token, user: &crate::param::NewUser) -> crate::Result<crate::data::NewUser> {
        let data = crate::Api::post(&self.config, "/users", user, Some(auth)).await?;

        let crate::data::Data::NewUser(user) = data;

        Ok(user)
    }

    /**
     * List users.
     */
    pub async fn all(&self, auth: &crate::data::Token, params: &crate::param::Users) -> crate::Result<crate::Pager<crate::data::User>> {
        crate::Api::get(&self.config, "/users", params, Some(auth)).await
    }

    /**
     * Delete a user.
     */
    pub async fn delete(&self, auth: &crate::data::Token, id: u32) -> crate::Result<()> {
        let _: crate::data::Empty = crate::Api::delete(&self.config, &format!("/users/{}", id), (), Some(auth)).await?;

        Ok(())
    }

    /**
     * Get a user.
     */
    pub async fn get(&self, auth: &crate::data::Token, id: u32) -> crate::Result<crate::data::User> {
        crate::Api::get(&self.config, &format!("/users/{}", id), (), Some(auth)).await
    }

    /**
     * Update a user.
     */
    pub async fn update(&self, auth: &crate::data::Token, id: u32, params: &crate::param::User) -> crate::Result<()> {
        let _: crate::data::Empty = crate::Api::put(&self.config, &format!("/users/{}", id), params, Some(auth)).await?;

        Ok(())
    }

    /**
     * Register a user.
     */
    pub async fn register(&self, params: &crate::param::Register) -> crate::Result<()> {
        let _: crate::data::Empty = crate::Api::post(&self.config, "/users/register", params, None).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn create() {
        let (api, token) = crate::test::api();
        let param = crate::param::NewUser {
            email: "test@example.org".to_string(),
            password: "123456".to_string(),
            username: "test".to_string(),

            .. Default::default()
        };
        let user = tokio_test::block_on(
            api.users.create(&token, &param)
        );
        assert!(user.is_ok());
    }

    #[test]
    fn all() {
        let (api, token) = crate::test::api();

        let users = tokio_test::block_on(
            api.users.all(&token, &crate::param::Users::default())
        );
        assert!(users.is_ok());
    }

    #[test]
    fn delete() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.users.delete(&token, 999)
        );
        assert!(status.is_ok());
    }

    #[test]
    fn get() {
        let (api, token) = crate::test::api();

        let user = tokio_test::block_on(
            api.users.get(&token, 1)
        );
        assert!(user.is_ok());
    }

    #[test]
    fn update() {
        let (api, token) = crate::test::api();
        let param = crate::param::User {
            id: 1,
            video_quota_daily: Some(100),

            .. Default::default()
        };
        let user = tokio_test::block_on(
            api.users.update(&token, 1, &param)
        );
        assert!(user.is_ok());
    }

    #[test]
    fn register() {
        let (api, _) = crate::test::api();
        let param = crate::param::Register {
            email: "test@example.org".to_string(),
            password: "123456".to_string(),
            username: "test".to_string(),

            .. Default::default()
        };
        let user = tokio_test::block_on(
            api.users.register(&param)
        );
        assert!(user.is_ok());
    }
}
