mod captions;
mod comments;
mod live;
mod ownership;

use captions::*;
use comments::*;
use live::*;
use ownership::*;

pub struct Videos {
    config: crate::Config,
    pub captions: Captions,
    pub comments: Comments,
    pub live: Live,
    pub ownership: Ownership,
}

impl Videos {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
            captions: Captions::new(config),
            comments: Comments::new(config),
            live: Live::new(config),
            ownership: Ownership::new(config),
        }
    }

    /**
     * List videos.
     */
    pub async fn all(&self, params: &crate::param::Videos) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: "/videos".to_string(),
            params: crate::Params::Query(params),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * List available video categories.
     */
    pub async fn categories(&self) -> crate::Result<std::collections::HashMap<u32, String>> {
        crate::Api::get(&self.config, "/videos/categories".into()).await
    }

    /**
     * List available video licences.
     */
    pub async fn licences(&self) -> crate::Result<std::collections::HashMap<u32, String>> {
        crate::Api::get(&self.config, "/videos/licences".into()).await
    }

    /**
     * List available video languages.
     */
    pub async fn languages(&self) -> crate::Result<std::collections::HashMap<String, String>> {
        crate::Api::get(&self.config, "/videos/languages".into()).await
    }

    /**
     * List available video privacy policies.
     */
    pub async fn privacies(&self) -> crate::Result<std::collections::HashMap<u32, String>> {
        crate::Api::get(&self.config, "/videos/privacies".into()).await
    }

    /**
     * Update a video.
     */
    pub async fn update(&self, auth: &crate::data::Token, id: &str, params: &crate::param::Video) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/{}", id),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::put::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Get a video.
     */
    pub async fn get(&self, id: &str) -> crate::Result<crate::data::Video> {
        crate::Api::get(&self.config, format!("/videos/{}", id).into()).await
    }

    /**
     * Delete a video.
     */
    pub async fn delete(&self, auth: &crate::data::Token, id: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/videos/{}", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Get complete video description.
     */
    pub async fn description(&self, id: &str) -> crate::Result<String> {
        let description: crate::data::Description = crate::Api::get(&self.config, format!("/videos/{}/description", id).into()).await?;

        Ok(description.description)
    }

    /**
     * Add a view to a video.
     */
    pub async fn add_view(&self, id: &str) -> crate::Result<()> {
        crate::Api::post::<crate::data::Empty, _>(&self.config, format!("/videos/{}/views", id).into())
            .await?
            .into()
    }

    /**
     * Set watching progress of a video.
     */
    pub async fn set_watching(&self, auth: &crate::data::Token, id: &str, current_time: u32) -> crate::Result<()> {
        let params = crate::param::Watching {
            current_time,
        };

        let request = crate::Request {
            path: format!("/videos/{}/watching", id),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::put::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Upload a video.
     */
    pub async fn upload(&self, auth: &crate::data::Token, videofile: &str, params: &crate::param::NewVideo) -> crate::Result<crate::data::NewContent> {
        let request = crate::Request {
            path: "/videos/upload".to_string(),
            params: crate::Params::upload(params, "videofile", videofile)?,
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Import a video.
     */
    pub async fn import(&self, auth: &crate::data::Token, params: &crate::param::Import) -> crate::Result<crate::data::NewContent> {
        let request = crate::Request {
            path: "/videos/imports".to_string(),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Like/dislike a video.
     */
    pub async fn rate(&self, auth: &crate::data::Token, id: &str, rate: crate::param::Rating) -> crate::Result<()> {
        let params = crate::param::Ratings {
            rating: Some(rate),

            .. Default::default()
        };

        let request = crate::Request {
            path: format!("/videos/{}/rate", id),
            params: crate::Params::Json(params),
            auth: Some(auth.clone()),
        };

        crate::Api::put::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn all() {
        let (api, _) = crate::test::api();

        let videos = tokio_test::block_on(
            api.videos.all(&crate::param::Videos::default())
        );
        assert!(videos.is_ok());
    }

    #[test]
    fn categories() {
        let (api, _) = crate::test::api();

        let categories = tokio_test::block_on(
            api.videos.categories()
        );
        assert!(categories.is_ok());
    }

    #[test]
    fn licences() {
        let (api, _) = crate::test::api();

        let licences = tokio_test::block_on(
            api.videos.licences()
        );
        assert!(licences.is_ok());
    }

    #[test]
    fn languages() {
        let (api, _) = crate::test::api();

        let languages = tokio_test::block_on(
            api.videos.languages()
        );
        assert!(languages.is_ok());
    }

    #[test]
    fn privacies() {
        let (api, _) = crate::test::api();

        let privacies = tokio_test::block_on(
            api.videos.privacies()
        );
        assert!(privacies.is_ok());
    }

    #[test]
    fn update() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.update(&token, "1cb3e9c4-2da6-4af3-804e-d4675c18e128", &crate::param::Video::default())
        );
        assert!(status.is_ok());
    }

    #[test]
    fn get() {
        let (api, _) = crate::test::api();

        let video = tokio_test::block_on(
            api.videos.get("1cb3e9c4-2da6-4af3-804e-d4675c18e128")
        );
        assert!(video.is_ok());
    }

    #[test]
    fn delete() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.delete(&token, "1cb3e9c4-2da6-4af3-804e-d4675c18e128")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn description() {
        let (api, _) = crate::test::api();

        let description = tokio_test::block_on(
            api.videos.description("1cb3e9c4-2da6-4af3-804e-d4675c18e128")
        );
        assert!(description.is_ok());
    }

    #[test]
    fn add_view() {
        let (api, _) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.add_view("1cb3e9c4-2da6-4af3-804e-d4675c18e128")
        );
        assert!(status.is_ok());
    }

    #[test]
    fn set_watching() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.set_watching(&token, "1cb3e9c4-2da6-4af3-804e-d4675c18e128", 10)
        );
        assert!(status.is_ok());
    }

    #[test]
    fn upload() {
        let (api, token) = crate::test::api();
        let params = crate::param::NewVideo {
            channel_id: "58edd166-dab0-4a1e-86e3-85778b78ba77".to_string(),
            name: "test".to_string(),

            .. Default::default()
        };

        let video = tokio_test::block_on(
            api.videos.upload(&token, "fixtures/video.mp4", &params)
        );
        assert!(video.is_ok());
    }

    #[test]
    fn import() {
        let (api, token) = crate::test::api();
        let params = crate::param::Import {
            video: crate::param::NewVideo {
                channel_id: "58edd166-dab0-4a1e-86e3-85778b78ba77".to_string(),
                name: "test".to_string(),

                .. Default::default()
            },
            target_url: Some("http://example.org/video.mp4".to_string()),

            .. Default::default()
        };

        let video = tokio_test::block_on(
            api.videos.import(&token, &params)
        );
        assert!(video.is_ok());
    }

    #[test]
    fn rate() {
        let (api, token) = crate::test::api();

        let status = tokio_test::block_on(
            api.videos.rate(&token, "1cb3e9c4-2da6-4af3-804e-d4675c18e128", crate::param::Rating::Like)
        );
        assert!(status.is_ok());
    }
}
