#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub url: String,
    pub name: String,
    pub host: String,
    pub avatar: Option<Avatar>,
    pub id: u32,
    pub host_redundancy_allowed: bool,
    pub following_count: u32,
    pub followers_count: u32,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub display_name: String,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub path: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
}

impl Api {
    pub async fn account(&self, name: &str) -> crate::Result<Account> {
        self.get(&format!("/accounts/{}", name), &crate::param::None).await
    }

    pub async fn accounts(&self, params: crate::param::Accounts) -> crate::Result<crate::Pager<Account>> {
        self.get("/accounts", &params).await
    }
}
