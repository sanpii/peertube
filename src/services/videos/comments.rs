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
            params: pagination,
            auth: None,
            form: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Create a thread.
     */
    pub async fn create(&self, auth: &crate::data::Token, video_id: &str, text: &str) -> crate::Result<crate::data::Comment> {
        let request = crate::Request {
            path: format!("/videos/{}/comment-threads", video_id),
            params: crate::param::Comment {
                text: text.to_string()
            },
            auth: Some(auth.clone()),
            form: None,
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Get a thread.
     */
    pub async fn get(&self, video_id: &str, thread_id: u32) -> crate::Result<crate::data::Thread> {
        let request = crate::Request {
            path: format!("/videos/{}/comment-threads/{}", video_id, thread_id),
            params: (),
            auth: None,
            form: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Reply to a thread of a video.
     */
    pub async fn reply(&self, auth: &crate::data::Token, video_id: &str, comment_id: u32, text: &str) -> crate::Result<crate::data::Comment> {
        let request = crate::Request {
            path: format!("/videos/{}/comments/{}", video_id, comment_id),
            params:  crate::param::Comment {
                text: text.to_string(),
            },
            auth: Some(auth.clone()),
            form: None,
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
            params: (),
            auth: Some(auth.clone()),
            form: None,
        };

        crate::Api::get::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn all() {
        let (api, _) = crate::test::api();

        let comments = tokio_test::block_on(
            api.videos.comments.all("601539e5-6bf9-42eb-9f5b-b9ede7635bda", &crate::param::Pagination::default())
        );
        assert!(comments.is_ok());
    }

    #[test]
    fn create() {
        let (api, token) = crate::test::api();

        let comment = tokio_test::block_on(
            api.videos.comments.create(&token, "601539e5-6bf9-42eb-9f5b-b9ede7635bda", "new comment")
        );
        assert!(comment.is_ok());
    }

    #[test]
    fn get() {
        let (api, _) = crate::test::api();

        let comment = tokio_test::block_on(
            api.videos.comments.get("601539e5-6bf9-42eb-9f5b-b9ede7635bda", 12005)
        );
        assert!(comment.is_ok());
    }

    #[test]
    fn reply() {
        let (api, token) = crate::test::api();

        let comment = tokio_test::block_on(
            api.videos.comments.reply(&token, "601539e5-6bf9-42eb-9f5b-b9ede7635bda", 12005, "reply")
        );
        assert!(comment.is_ok());
    }

    #[test]
    fn delete() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.comments.delete(&token, "601539e5-6bf9-42eb-9f5b-b9ede7635bda", 12005)
        );
        assert!(status.is_ok());
    }
}
