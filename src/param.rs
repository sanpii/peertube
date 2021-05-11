#[derive(Debug, Default, serde::Serialize)]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Videos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_one_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<VideoFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_live: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_one_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_one_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_all_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_one_of: Option<String>,
    #[serde(flatten)]
    pub pagination: Pagination,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_stats: Option<bool>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub role: crate::data::Role,
    pub username: String,
    pub video_quota: i32,
    pub video_quota_daily: i32,
    pub admin_flags: AdminFlag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::data::AbuseState>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Avatar {
    pub avatarfile: String,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Subscription {
    pub uri: String,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
}

#[derive(Debug, Default, serde::Serialize)]
pub(crate) struct Notifications {
    pub ids: Vec<u32>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub abuse_as_moderator: NotificationSettingsValue,
    pub auto_instance_following: NotificationSettingsValue,
    pub blacklist_on_my_video: NotificationSettingsValue,
    pub comment_mention: NotificationSettingsValue,
    pub my_video_import_finished: NotificationSettingsValue,
    pub my_video_published: NotificationSettingsValue,
    pub new_comment_on_my_video: NotificationSettingsValue,
    pub new_follow: NotificationSettingsValue,
    pub new_instance_follower: NotificationSettingsValue,
    pub new_user_registration: NotificationSettingsValue,
    pub new_video_from_subscription: NotificationSettingsValue,
    pub video_auto_blacklist_as_moderator: NotificationSettingsValue,
}

#[derive(Debug, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum NotificationSettingsValue {
    None = 0,
    Web = 1,
    Email = 2,
    Both = 3,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct History {
    pub before_date: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originally_published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previewfile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_update: Option<crate::data::ScheduledUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnailfile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twait_transcoding: Option<String>,
}

#[derive(Debug, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Privacy {
    Public = 1,
    Unlisted = 2,
    Private = 3,
    Internal = 4,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Watching {
    pub current_time: u32,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewVideo {
    pub channel_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originally_published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previewfile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_update: Option<crate::data::ScheduledUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnailfile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twait_transcoding: Option<String>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Import {
    #[serde(flatten)]
    pub video: NewVideo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torrentfile: Option<String>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    #[serde(flatten)]
    pub video: NewVideo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_live: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previewfile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_replay: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_replay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_live: Option<bool>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Caption {
    pub captionfile: String,
}
