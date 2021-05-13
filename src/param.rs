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

#[derive(Clone, Copy, Debug, serde::Serialize)]
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

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<String>,
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

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_videos_support_update: Option<bool>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Comment {
    pub text: String,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnailfile: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_channel_id: Option<u32>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnailfile: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_channel_id: Option<u32>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistElement {
    pub video_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestmap: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestmap: Option<u64>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reorder {
    pub insert_after_position: u32,
    pub start_position: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reorder_length: Option<u32>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Elements {
    pub video_ids: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Ownership {
    pub username: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Redundancies {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub target: MirrorDirection,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MirrorDirection {
    MyVideos,
    RemoteVideos,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Redundancy {
    pub video_id: u32,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchVideos {
    #[serde(flatten)]
    pub pagination: Videos,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_min: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_max: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originally_published_end_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originally_published_start_date: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub search: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_target: Option<SearchTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::DateTime<chrono::offset::Utc>>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchChannels {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub search: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_target: Option<SearchTarget>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SearchTarget {
    Local,
    SearchIndex,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Abuses {
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AbuseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_reason: Option<crate::data::AbusePredefinedReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_reportee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_reporter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_video: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_video_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::data::AbuseState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_is: Option<VideoStatus>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AbuseType {
    Video,
    Comment,
    Account,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum VideoStatus {
    Deleted,
    Blacklisted,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Abuse {
    pub reason: String,
    #[serde(flatten)]
    pub abuse_element: AbuseElement,
    pub predefined_reasons: Vec<crate::data::AbusePredefinedReason>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AbuseElement {
    Account(AbusedAccount),
    Comment(AbusedComment),
    Video(AbusedVideo),
}

#[derive(Debug, serde::Serialize)]
pub struct AbusedAccount {
    pub id: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct AbusedComment {
    pub id: u32,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbusedVideo {
    pub id: u32,
    pub start_at: u32,
    pub end_at: u32,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbuseSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::data::AbuseState>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct AbuseMessage {
    pub message: String,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoBlacklists {
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<BlacklistType>,
}

#[derive(Debug, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum BlacklistType {
    Manual = 1,
    Automatic = 2,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Following {
    pub host: String,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct Followings {
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<ActorType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::data::FollowState>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActorType {
    Person,
    Application,
    Group,
    Service,
    Organization,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RedundancySetting {
    pub redundancy_allowed: bool,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Plugin {
    pub npm_name: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginSettings {
    //pub settings: (),
}

#[derive(Debug, Default, serde::Serialize)]
pub struct Jobs {
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<crate::data::JobType>,
}
