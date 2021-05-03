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
pub struct AccountSummary {
    pub avatar: Option<Avatar>,
    pub display_name: String,
    pub host: String,
    pub id: u32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub path: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub account: AccountSummary,
    pub blacklisted: Option<bool>,
    pub blacklisted_reason: Option<String>,
    pub category: Category,
    pub channel: ChannelSummary,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: String,
    pub dislikes: u32,
    pub duration: u32,
    pub embed_path: String,
    pub id: u32,
    pub is_like: Option<bool>,
    pub is_local: bool,
    pub language: Language,
    pub license: Option<License>,
    pub likes: u32,
    pub name: String,
    pub nsfw: bool,
    pub originally_published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub preview_path: String,
    pub privacy: Privacy,
    pub published_at: chrono::DateTime<chrono::offset::Utc>,
    pub scheduled_update: Option<ScheduledUpdate>,
    pub state: Option<State>,
    pub thumbnail_path: String,
    pub update_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub user_history: Option<UserHistory>,
    pub uuid: String,
    pub views: u32,
    pub wait_transcoding: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: u32,
    pub label: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSummary {
    pub avatar: Option<Avatar>,
    pub display_name: String,
    pub host: Option<String>,
    pub id: u32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub id: Option<String>,
    pub label: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: u32,
    pub label: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Privacy {
    pub id: u32,
    pub label: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ScheduledUpdate {
    update_at: String,
    privacy: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub id: u32,
    pub label: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserHistory {
    current_time: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub description: Option<String>,
    pub display_name: String,
    pub is_local: bool,
    pub owner_account: Account,
}
