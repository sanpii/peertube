pub mod config;

pub use config::Config;

#[derive(Debug)]
pub(crate) struct Empty;

impl<'de> serde::Deserialize<'de> for Empty {
    fn deserialize<D: serde::Deserializer<'de>>(_: D) -> Result<Empty, D::Error> {
        Ok(Empty)
    }
}

impl From<Empty> for crate::Result<()> {
    fn from(_: Empty) -> Self {
        Ok(())
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
    pub host_redundancy_allowed: Option<bool>,
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
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::offset::Utc>>,
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
    pub url: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Language {
    pub id: Option<String>,
    pub label: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct License {
    pub id: u32,
    pub label: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Privacy {
    pub id: u32,
    pub label: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ScheduledUpdate {
    update_at: String,
    privacy: u32,
}

#[derive(Debug, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Deserialize)]
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
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
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

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
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
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
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
    pub predefined_reasons: Vec<AbusePredefinedReason>,
    pub reporter_account: Account,
    pub state: AbuseState,
    pub moderation_comment: String,
    pub video: VideoInfo,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Debug, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum AbuseState {
    Pending = 1,
    Rejected = 2,
    Accepted = 3,
}

#[derive(Debug, serde::Deserialize)]
pub struct VideoInfo {
    pub id: u32,
    pub uuid: String,
    pub name: String,
    pub channel: Option<ChannelSummary>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AbusePredefinedReason {
    ViolentOrAbusive,
    HatefulOrAbusive,
    SpamOrMisleading,
    Privacy,
    Rights,
    ServerRules,
    Thumbnails,
    Captions,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub avatar: Option<Avatar>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: Option<String>,
    pub display_name: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub host: String,
    pub host_redundancy_allowed: Option<bool>,
    pub id: u32,
    pub is_local: bool,
    pub name: String,
    pub owner_account: Option<Account>,
    pub support: Option<String>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    account: Option<AccountSummary>,
    //pub actor_follow: Option<>,
    pub comment: Option<CommentSummary>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub id: u32,
    pub read: bool,
    pub r#type: UserNotificationType,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub video: Option<VideoInfo>,
    //pub video_abuse: Option<>,
    //pub video_blacklist: Option<>,
    //pub video_import: Option<>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentSummary {
    pub account: AccountSummary,
    pub id: u32,
    pub thread_id: u32,
    pub video: VideoInfo,
}

#[derive(Debug, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum UserNotificationType {
    NewVideoFromSubscription = 1,
    NewCommentOnMyVideo = 2,
    NewAbuseForModerators = 3,
    BlacklistOnMyVideo = 4,
    UnblacklistOnMyVideo = 5,
    MyVideoPublished = 6,
    MyVideoImportSuccess = 7,
    MyVideoImportError = 8,
    NewUserRegistration = 9,
    NewFollow = 10,
    CommentMention = 11,
    VideoAutoBlacklistForModerators = 12,
    NewInstanceFollower = 13,
    AutoInstanceFollowing = 14,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Description {
    pub description: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NewContent {
    Abuse(NewAbuse),
    AbuseMessage(NewAbuseMessage),
    Video(NewVideo),
    #[serde(rename = "videoChannel")]
    Channel(NewChannel),
    Comment(Comment),
    #[serde(rename = "videoPlaylist")]
    Playlist(NewPlaylist),
    #[serde(rename = "videoPlaylistElement")]
    PlaylistElement(NewPlaylistElement),
}

#[derive(Debug, serde::Deserialize)]
pub struct NewAbuse {
    pub id: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct NewAbuseMessage {
    pub id: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct NewVideo {
    pub id: u32,
    pub uuid: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct NewChannel {
    pub id: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct NewPlaylist {
    pub id: u32,
    pub uuid: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct NewPlaylistElement {
    pub id: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    pub rtmp_url: String,
    pub stream_key: String,
    pub save_replay: bool,
    pub permanent_live: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Caption {
    pub caption_path: String,
    pub language: Language,
}

#[derive(Debug, serde::Deserialize)]
pub struct Thread {
    pub comment: Comment,
    pub children: Vec<Comment>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub account: Option<Account>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub id: u32,
    pub in_reply_to_comment_id: Option<u32>,
    pub text: String,
    pub thread_id: u32,
    pub total_replies: u32,
    pub total_replies_from_video_author: u32,
    pub is_deleted: bool,
    pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
    pub video_id: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: Option<String>,
    pub display_name: String,
    pub id: u32,
    pub is_local: bool,
    pub owner_account: AccountSummary,
    pub privacy: Privacy,
    pub thumbnail_path: Option<String>,
    pub r#type: PlaylistType,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub uuid: String,
    pub video_channel: Option<ChannelSummary>,
    pub videos_length: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct PlaylistType {
    pub id: PlaylistId,
    pub label: String,
}

#[derive(Debug, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum PlaylistId {
    Regular = 1,
    WatchLater = 2,
}

#[derive(Debug, serde::Deserialize)]
pub struct Ownership {
    pub id: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Redundancy {
    pub id: u32,
    pub name: String,
    pub url: String,
    pub uuid: String,
    pub redundancies: Redundancies,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Redundancies {
    pub files: Vec<FileRedundancyInformation>,
    pub streaming_playlists: Vec<FileRedundancyInformation>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRedundancyInformation {
    pub id: u32,
    pub file_url: String,
    pub strategy: Strategy,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub expires_on: chrono::DateTime<chrono::offset::Utc>,
    pub size: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Strategy {
    Manual,
    MostViews,
    Trending,
    RecentlyAdded,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbuseMessage {
    pub id: u32,
    pub message: String,
    pub by_moderator: bool,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub account: AccountSummary,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoBlacklist {
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: String,
    pub dislikes: u32,
    pub duration: u32,
    pub id: u32,
    pub likes: u32,
    pub name: String,
    pub nfsw: bool,
    pub uuid: String,
    pub video_id: u32,
    pub views: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockedServer {
    pub by_account: Account,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub blocked_server: Server,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub host: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockedAccount {
    pub by_account: Account,
    pub blocked_account: Account,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccountName {
    pub account_name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    pub id: u32,
    pub score: u32,
    pub state: FollowState,
    pub follower: Account,
    pub following: Account,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FollowState {
    Pending,
    Accepted,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub name: String,
    pub r#type: u32,
    pub latest_version: String,
    pub version: String,
    pub enabled: bool,
    pub uninstalled: bool,
    pub peertube_engine: String,
    pub description: String,
    pub homepage: String,
    //pub settings: (),
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailablePlugin {
    pub npm_name: String,
    pub description: String,
    pub homepage: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub popularity: f32,
    pub latest_version: String,
    pub installed: bool,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicSettings {
    //pub public_settings: (),
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredSettings {
}
