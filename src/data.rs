#[derive(Debug)]
pub(crate) struct Empty;

impl<'de> serde::Deserialize<'de> for Empty {
    fn deserialize<D: serde::Deserializer<'de>>(_: D) -> Result<Empty, D::Error> {
        Ok(Empty)
    }
}

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
    pub description: Option<String>,
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
    pub id: Option<u32>,
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
pub(crate) struct OauthClient {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Data {
    NewUser(NewUser),
}

#[derive(Debug, serde::Deserialize)]
pub struct NewUser {
    pub account: AccountId,
    pub id: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct AccountId {
    pub id: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub plugin_auth: Option<String>,
    pub theme: String,
    pub email_verified: Option<bool>,
    pub nsfw_policy: String,
    pub web_torrent_enabled: bool,
    pub auto_play_video: bool,
    pub role: Role,
    pub role_label: String,
    pub video_quota: i64,
    pub video_quota_daily: i64,
    pub no_instance_config_warning_modal: bool,
    pub no_welcome_modal: bool,
    pub blocked: bool,
    pub blocked_reason: Option<String>,
    pub created_at: String,
    pub account: Account,
    pub video_channels: Vec<Channel>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub description: Option<String>,
    pub display_name: String,
    pub is_local: bool,
}

#[derive(Debug, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Role {
    Admin = 0,
    Moderator = 1,
    User = 2,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Import {
    pub id: u32,
    pub target_url: String,
    pub magnet_uri: Option<String>,
    pub torrent_name: Option<String>,
    pub state: State,
    pub error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub video: Video,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quota {
    pub video_quota_used: u64,
    pub video_quota_used_daily: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub video_id: u32,
    pub rating: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abuse {
    pub id: u32,
    pub reason: String,
    #[serde(default)]
    pub predefined_reasons: Vec<AbusePredefinedReasons>,
    pub reporter_account: Account,
    pub state: AbuseState,
    pub moderation_comment: String,
    pub video: VideoInfo,
    pub created_at: String,
}

#[derive(Debug, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum AbuseState {
    Pending = 1,
    Rejected = 2,
    Accepted = 3,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    pub id: u32,
    pub uuid: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AbusePredefinedReasons {
    ViolentOrAbusive,
    HatefulOrAbusive,
    SpamOrMisleading,
    Privacy,
    Rights,
    ServerRules,
    Thumbnails,
    Captions,
}
