#[derive(Debug, Default, serde::Serialize)]
pub struct Pagination {
    pub count: Option<usize>,
    pub sort: Option<String>,
    pub start: Option<usize>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Videos {
    pub category_one_of: Option<String>,
    pub count: Option<usize>,
    pub filter: Option<VideoFilter>,
    pub is_live: Option<bool>,
    pub language_one_of: Option<String>,
    pub license_one_of: Option<String>,
    pub nsfw: Option<bool>,
    pub skip_count: Option<bool>,
    pub sort: Option<String>,
    pub tags_all_of: Option<String>,
    pub tags_one_of: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoFilter {
    Local,
    AllLocal,
    All,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channels {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub with_stats: Option<bool>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub rating: Option<Rating>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    Like,
    Dislike,
}

#[derive(Debug, Default, serde::Serialize)]
pub(crate) struct Auth {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub response_type: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub role: Option<crate::data::Role>,
    pub username: String,
    pub video_quota: i32,
    pub video_quota_daily: i32,
    pub admin_flags: AdminFlag,
    pub channel_name: String,
}

#[derive(Debug, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum AdminFlag {
    None = 0,
    BypassVideoBlacklist = 1,
}

impl Default for AdminFlag {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct Users {
    pub blocked: Option<bool>,
    pub search: Option<String>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_flags: Option<AdminFlag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::data::Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quota: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quota_daily: Option<i32>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    pub email: String,
    pub password: String,
    pub username: String,
    pub channel: Option<Channel>,
    pub display_name: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Me {
    pub auto_play_video: bool,
    #[serde(rename = "displayNSFW")]
    pub display_nsfw: DisplayNsfw,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DisplayNsfw {
    True,
    False,
    Both,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Abuse {
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<crate::data::AbuseState>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Avatar {
    pub avatarfile: String,
}
