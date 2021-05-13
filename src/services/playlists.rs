pub struct Playlists {
    config: crate::Config,
}

impl Playlists {
    pub(crate) fn new(config: &crate::Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /**
     * List available playlist privacy policies.
     */
    pub async fn privacies(&self) -> crate::Result<std::collections::HashMap<u32, String>> {
        crate::Api::get(&self.config, "/video-playlists/privacies".into()).await
    }

    /**
     * List video playlists.
     */
    pub async fn all(
        &self,
        pagination: &crate::param::Pagination,
    ) -> crate::Result<crate::Pager<crate::data::Playlist>> {
        let request = crate::Request {
            path: "/video-playlists".to_string(),
            params: crate::Params::Query(pagination),
            auth: None,
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Create a video playlist.
     */
    pub async fn create(
        &self,
        auth: &crate::data::Token,
        playlist: &crate::param::Playlist,
    ) -> crate::Result<crate::data::NewContent> {
        let request = crate::Request {
            path: "/video-playlists".to_string(),
            params: crate::Params::multipart(playlist)?,
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Get a video playlist.
     */
    pub async fn get(
        &self,
        auth: Option<&crate::data::Token>,
        id: &str,
    ) -> crate::Result<crate::data::Playlist> {
        let request = crate::Request {
            path: format!("/video-playlists/{}", id),
            params: crate::Params::none(),
            auth: auth.cloned(),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Update a video playlist.
     */
    pub async fn update(
        &self,
        auth: &crate::data::Token,
        id: &str,
        params: &crate::param::PlaylistSetting,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-playlists/{}", id),
            params: crate::Params::multipart(params)?,
            auth: Some(auth.clone()),
        };

        crate::Api::put(&self.config, request).await
    }

    /**
     * Delete a video playlist.
     */
    pub async fn delete(&self, auth: &crate::data::Token, id: &str) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-playlists/{}", id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }

    /**
     * List videos of a playlist.
     */
    pub async fn videos(
        &self,
        auth: Option<&crate::data::Token>,
        id: &str,
    ) -> crate::Result<crate::Pager<crate::data::Video>> {
        let request = crate::Request {
            path: format!("/video-playlists/{}/videos", id),
            params: crate::Params::none(),
            auth: auth.cloned(),
        };

        crate::Api::get(&self.config, request).await
    }

    /**
     * Add a video in a playlist.
     */
    pub async fn add_video(
        &self,
        auth: &crate::data::Token,
        id: &str,
        element: &crate::param::PlaylistElement,
    ) -> crate::Result<crate::data::NewContent> {
        let request = crate::Request {
            path: format!("/video-playlists/{}/videos", id),
            params: crate::Params::Json(element),
            auth: Some(auth.clone()),
        };

        crate::Api::post(&self.config, request).await
    }

    /**
     * Reorder a playlist.
     */
    pub async fn reorder(
        &self,
        auth: &crate::data::Token,
        id: &str,
        reorder: &crate::param::Reorder,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-playlists/{}/videos/reorder", id),
            params: crate::Params::Json(reorder),
            auth: Some(auth.clone()),
        };

        crate::Api::post::<crate::data::Empty, _>(&self.config, request)
            .await?
            .into()
    }

    /**
     * Update a playlist element.
     */
    pub async fn update_video(
        &self,
        auth: &crate::data::Token,
        id: &str,
        element: &crate::param::PlaylistElement,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-playlists/{}/videos/{}", id, element.video_id),
            params: crate::Params::Json(element),
            auth: Some(auth.clone()),
        };

        crate::Api::put(&self.config, request).await
    }

    /**
     * Delete an element from a playlist.
     */
    pub async fn delete_video(
        &self,
        auth: &crate::data::Token,
        id: &str,
        element_id: &str,
    ) -> crate::Result<()> {
        let request = crate::Request {
            path: format!("/video-playlists/{}/videos/{}", id, element_id),
            params: crate::Params::none(),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }

    /**
     * Check video exists in my playlists.
     */
    pub async fn exists<T: ToString>(
        &self,
        auth: &crate::data::Token,
        element_id: &[T],
    ) -> crate::Result<()> {
        let params = crate::param::Elements {
            video_ids: element_id.iter().map(|x| x.to_string()).collect(),
        };

        let request = crate::Request {
            path: "/video-playlists/videos-exist".to_string(),
            params: crate::Params::Query(params),
            auth: Some(auth.clone()),
        };

        crate::Api::delete(&self.config, request).await
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn privacies() {
        let (api, _) = crate::test::api().await;

        let privacies = api.playlists.privacies().await;

        assert!(privacies.is_ok());
    }

    #[tokio::test]
    async fn all() {
        let (api, _) = crate::test::api().await;

        let playlists = api
            .playlists
            .all(&crate::param::Pagination::default())
            .await;

        assert!(playlists.is_ok());
    }

    #[tokio::test]
    async fn create() {
        let (api, token) = crate::test::api().await;
        let playlist = crate::param::Playlist {
            display_name: "playlist".to_string(),

            ..Default::default()
        };

        let status = api.playlists.create(&token, &playlist).await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn get() {
        let (api, _) = crate::test::api().await;

        let playlist = api
            .playlists
            .get(None, "cdd3d948-5262-4460-b427-ac2e8003e6f7")
            .await;

        assert!(playlist.is_ok());
    }

    #[tokio::test]
    async fn update() {
        let (api, token) = crate::test::api().await;
        let params = crate::param::PlaylistSetting {
            display_name: Some("playlist 2".to_string()),

            ..Default::default()
        };

        let status = api
            .playlists
            .update(&token, "cdd3d948-5262-4460-b427-ac2e8003e6f7", &params)
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete() {
        let (api, token) = crate::test::api().await;

        let status = api
            .playlists
            .delete(&token, "cdd3d948-5262-4460-b427-ac2e8003e6f7")
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn videos() {
        let (api, _) = crate::test::api().await;

        let playlist = api
            .playlists
            .get(None, "cdd3d948-5262-4460-b427-ac2e8003e6f7")
            .await;

        assert!(playlist.is_ok());
    }

    #[tokio::test]
    async fn add_video() {
        let (api, token) = crate::test::api().await;
        let params = crate::param::PlaylistElement {
            video_id: 10,

            ..Default::default()
        };

        let element = api
            .playlists
            .add_video(&token, "cdd3d948-5262-4460-b427-ac2e8003e6f7", &params)
            .await;

        assert!(element.is_ok());
    }

    #[tokio::test]
    async fn reorder() {
        let (api, token) = crate::test::api().await;
        let params = crate::param::Reorder {
            insert_after_position: 2,
            start_position: 1,

            ..Default::default()
        };

        let status = api
            .playlists
            .reorder(&token, "cdd3d948-5262-4460-b427-ac2e8003e6f7", &params)
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn update_video() {
        let (api, token) = crate::test::api().await;
        let element = crate::param::PlaylistElement {
            video_id: 10,

            ..Default::default()
        };

        let status = api
            .playlists
            .update_video(&token, "cdd3d948-5262-4460-b427-ac2e8003e6f7", &element)
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn delete_video() {
        let (api, token) = crate::test::api().await;

        let status = api
            .playlists
            .delete_video(&token, "cdd3d948-5262-4460-b427-ac2e8003e6f7", "10")
            .await;

        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn exists() {
        let (api, token) = crate::test::api().await;

        let status = api.playlists.exists(&token, &["10"]).await;

        assert!(status.is_ok());
    }
}
