pub struct Comments {
    config: crate::Config,
}

impl Comments {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List threads of a video.
     */
    pub async fn all(&self, video_id: &str, pagination: &crate::param::Pagination) -> crate::Result<crate::Pager<crate::data::Comment>> {
        let request = crate::Request {
            path: format!("/videos/{}/comment-threads", video_id),
            params: crate::Params::Query(pagination),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Create a thread.
     */
    pub async fn create(&self, auth: &crate::data::Token, video_id: &str, text: &str) -> crate::Result<crate::data::Comment> {
        let request = crate::Request {
            path: format!("/videos/{}/comment-threads", video_id),
            params: crate::Params::Json(crate::param::Comment {
                text: text.to_string()
            }),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Get a thread.
     */
    pub async fn get(&self, video_id: &str, thread_id: u32) -> crate::Result<crate::data::Thread> {
        let request = crate::Request {
            path: format!("/videos/{}/comment-threads/{}", video_id, thread_id),
            params: crate::Params::none(),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Reply to a thread of a video.
     */
    pub async fn reply(&self, auth: &crate::data::Token, video_id: &str, comment_id: u32, text: &str) -> crate::Result<crate::data::Comment> {
        let params = crate::param::Comment {
            text: text.to_string(),
        };

        let request = crate::Request {
            path: format!("/videos/{}/comments/{}", video_id, comment_id),
            params:  crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        match crate::Api::post(&self.config, request).await? {
            crate::data::NewContent::Comment(comment) => Ok(comment),
            _ => unreachable!(),
        }
    }

    /**
     * Delete a comment or a reply.
     */
    pub async fn delete(&self, auth: &crate::data::Token, video_id: &str, comment_id: u32) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/{}/comments/{}", video_id, comment_id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::get::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn all() {
        let (api, _) = crate::test::api().await;

        let comments = api.videos.comments.all("601539e5-6bf9-42eb-9f5b-b9ede7635bda", &crate::param::Pagination::default())
            .await;

        assert!(comments.is_ok());
    }

    #[tokio::test]
    async fn create() {
        let (api, token) = crate::test::api().await;

        let comment = api.videos.comments.create(&token, "601539e5-6bf9-42eb-9f5b-b9ede7635bda", "new comment")
            .await;

        assert!(comment.is_ok());
    }

    #[tokio::test]
    async fn get() {
        let (api, _) = crate::test::api().await;

        let comment = api.videos.comments.get("601539e5-6bf9-42eb-9f5b-b9ede7635bda", 12005)
            .await;

        assert!(comment.is_ok());
    }

    #[tokio::test]
    async fn reply() {
        let (api, token) = crate::test::api().await;

        let comment = api.videos.comments.reply(&token, "601539e5-6bf9-42eb-9f5b-b9ede7635bda", 12005, "reply")
            .await;

        assert!(comment.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api.videos.comments.delete(&token, "601539e5-6bf9-42eb-9f5b-b9ede7635bda", 12005)
            .await;

        assert!(status.is_ok());
    }
}
