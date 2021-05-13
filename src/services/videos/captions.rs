pub struct Captions {
    config: crate::Config,
}

impl Captions {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List captions of a video.
     */
    pub async fn all(&self, video_id: &str) -> crate::Result<crate::Pager<crate::data::Caption>> {
        crate::Api::get(
            &self.config,
            format!("/videos/{}/captions", video_id).into(),
        )
        .await
    }

    /**
     * Add or replace a video caption.
     */
    pub async fn add(
        &self,
        auth: &crate::data::Token,
        video_id: &str,
        language: &str,
        captionfile: &str,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/{}/captions/{}", video_id, language),
            params: crate::Params::upload((), "captionfile", captionfile)?,
            auth: Some(auth.clone()),
        };

        crate::Api::put::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Delete a video caption.
     */
    pub async fn delete(
        &self,
        auth: &crate::data::Token,
        video_id: &str,
        language: &str,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/{}/captions/{}", video_id, language),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn all() {
        let (api, _) = crate::test::api().await;

        let captions = api
            .videos
            .captions
            .all("1cb3e9c4-2da6-4af3-804e-d4675c18e128")
            .await;

        assert!(captions.is_ok());
    }

    #[tokio::test]
    async fn add() {
        let (api, token) = crate::test::api().await;

        let status = api
            .videos
            .captions
            .add(
                &token,
                "1cb3e9c4-2da6-4af3-804e-d4675c18e128",
                "fr",
                "fixtures/caption.srt",
            )
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api
            .videos
            .captions
            .delete(&token, "1cb3e9c4-2da6-4af3-804e-d4675c18e128", "fr")
            .await;

        assert!(status.is_ok());
    }
}
