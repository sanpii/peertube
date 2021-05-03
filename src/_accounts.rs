impl Api {
    pub async fn account(&self, name: &str) -> crate::Result<crate::Account> {
        self.get(&format!("/accounts/{}", name), (), None).await
    }

    pub async fn account_videos(&self, name: &str, params: &crate::param::Videos) -> crate::Result<crate::Pager<crate::Video>> {
        self.get(&format!("/accounts/{}/videos", name), params, None).await
    }

    pub async fn accounts(&self, params: &crate::param::Accounts) -> crate::Result<crate::Pager<crate::Account>> {
        self.get("/accounts", params, None).await
    }

    pub async fn account_video_channels(&self, name: &str, params: &crate::param::Channels) -> crate::Result<crate::Pager<crate::Channel>> {
        self.get(&format!("/accounts/{}/video-channels", name), params, None).await
    }
}
