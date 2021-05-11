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
        crate::Api::get(&self.config, format!("/videos/{}/captions", video_id).into()).await
    }

    /**
     * Add or replace a video caption.
     */
    pub async fn add(&self, auth: &crate::data::Token, video_id: &str, language: &str, captionfile: &str) -> crate::Result<()> {
        let part = reqwest::multipart::Part::bytes(std::fs::read(captionfile)?)
            .file_name(captionfile.to_string());

        let form = reqwest::multipart::Form::new()
            .part("captionfile", part);

        let request = crate::Request {
            path: format!("/videos/{}/captions/{}", video_id, language),
            params: (),
            auth: Some(auth.clone()),
            form: Some(form),
        };

        let _: crate::data::Empty = crate::Api::put(&self.config, request).await?;

        Ok(())
    }

    /**
     * Delete a video caption.
     */
    pub async fn delete(&self, auth: &crate::data::Token, video_id: &str, language: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/{}/captions/{}", video_id, language),
            params: (),
            auth: Some(auth.clone()),
            form: None,
        };

        let _: crate::data::Empty = crate::Api::delete(&self.config, request).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn all() {
        let (api, _) = crate::test::api();

        let captions = tokio_test::block_on(
            api.videos.captions.all("1cb3e9c4-2da6-4af3-804e-d4675c18e128")
        );
        assert!(captions.is_ok());
    }

    #[test]
    fn add() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.captions.add(&token, "1cb3e9c4-2da6-4af3-804e-d4675c18e128", "fr", "fixtures/caption.srt")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn delete() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.captions.delete(&token, "1cb3e9c4-2da6-4af3-804e-d4675c18e128", "fr")
        );
        assert!(status.is_ok());
    }
}
