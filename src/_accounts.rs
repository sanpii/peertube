impl Api {
    pub async fn account(&self, name: &str) -> crate::Result<crate::Account> {
        self.get(&format!("/accounts/{}", name), &crate::param::None).await
    }

    pub async fn account_videos(&self, name: &str) -> crate::Result<crate::Pager<crate::Video>> {
        self.get(&format!("/accounts/{}/videos", name), &crate::param::None).await
    }

    pub async fn accounts(&self, params: crate::param::Accounts) -> crate::Result<crate::Pager<crate::Account>> {
        self.get("/accounts", &params).await
    }
}
