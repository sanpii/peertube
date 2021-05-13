mod custom;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub instance: InstanceSummary,
    pub search: Search,
    pub plugin: Plugins,
    pub theme: Themes,
    pub email: Enabled,
    pub contact_form: Enabled,
    pub server_version: String,
    pub server_commit: String,
    pub signup: Signup,
    pub transcoding: Transcoding,
    pub live: Live,
    pub import: Import,
    pub auto_blacklist: AutoBlacklist,
    pub avatar: Avatar,
    pub video: Video,
    pub video_caption: VideoCaption,
    pub user: User,
    pub trending: Trending,
    pub tracker: Enabled,
    pub broadcast_message: BroadcastMessage,
}

#[derive(Debug, serde::Deserialize)]
pub struct Enabled {
    pub enabled: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceSummary {
    pub name: String,
    pub short_description: String,
    #[serde(rename = "isNSFW")]
    pub is_nsfw: bool,
    #[serde(rename = "defaultNSFWPolicy")]
    pub default_nsfw_poliy: String,
    pub default_client_route: String,
    pub customizations: Customizations,
}

#[derive(Debug, serde::Deserialize)]
pub struct Customizations {
    pub javascript: String,
    pub css: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub remote_uri: RemoteUri,
    pub search_index: SearchIndex,
}

#[derive(Debug, serde::Deserialize)]
pub struct RemoteUri {
    pub users: bool,
    pub anonymous: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchIndex {
    pub enabled: bool,
    pub url: String,
    pub disable_local_search: bool,
    pub is_default_search: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugins {
    pub registered: Vec<Plugin>,
    pub registered_external_auths: Vec<RegisteredExternalAuthConfig>,
    pub registered_id_and_pass_auths: Vec<RegisteredIdAndPassAuthConfig>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub description: String,
    pub client_scripts: std::collections::HashMap<String, ClientScript>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ClientScript {
    pub script: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredExternalAuthConfig {
    #[serde(rename = "npmName")]
    pub npm_name: String,
    pub name: String,
    pub version: String,
    pub auth_name: String,
    pub auth_display_name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredIdAndPassAuthConfig {
    #[serde(rename = "npmName")]
    pub npm_name: String,
    pub name: String,
    pub version: String,
    pub auth_name: String,
    pub weight: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Themes {
    pub registered: Vec<Theme>,
    pub default: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub name: String,
    pub version: String,
    pub css: Vec<String>,
    pub client_scripts: ClientScript,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signup {
    pub allowed: bool,
    #[serde(rename = "allowedForCurrentIP")]
    pub allowed_for_current_ip: bool,
    pub requires_email_verification: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transcoding {
    pub hls: Enabled,
    pub webtorrent: Enabled,
    pub enabled_resolutions: Vec<u32>,
    pub profile: String,
    pub available_profiles: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    pub enabled: bool,
    pub allow_replay: bool,
    pub max_duration: i32,
    pub max_instance_lives: i32,
    pub max_user_lives: i32,
    pub transcoding: LiveTranscoding,
    pub rtmp: Rtmp,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTranscoding {
    pub enabled: bool,
    pub enabled_resolutions: Vec<u32>,
    pub profile: String,
    pub available_profiles: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Rtmp {
    pub port: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Import {
    pub videos: Videos,
}

#[derive(Debug, serde::Deserialize)]
pub struct Videos {
    pub http: Enabled,
    pub torrent: Enabled,
}

#[derive(Debug, serde::Deserialize)]
pub struct AutoBlacklist {
    pub videos: AutoBlacklistVideos,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoBlacklistVideos {
    pub of_users: Enabled,
}

#[derive(Debug, serde::Deserialize)]
pub struct Avatar {
    pub file: File,
}

#[derive(Debug, serde::Deserialize)]
pub struct File {
    #[serde(default)]
    pub size: Size,
    pub extensions: Vec<String>,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Size {
    pub max: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Video {
    pub image: File,
    pub file: File,
}

#[derive(Debug, serde::Deserialize)]
pub struct VideoCaption {
    pub file: File,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub video_quota: i64,
    pub video_quota_daily: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Trending {
    pub videos: TrendingVideo,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingVideo {
    pub interval_days: u32,
    pub algorithms: Algorithms,
}

#[derive(Debug, serde::Deserialize)]
pub struct Algorithms {
    pub enabled: Vec<String>,
    pub default: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct BroadcastMessage {
    pub enabled: bool,
    pub message: String,
    pub level: String,
    pub dismissable: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct About {
    pub instance: Instance,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub name: String,
    pub short_description: String,
    pub description: String,
    pub terms: String,
    pub code_of_conduct: String,
    pub hardware_information: String,
    pub creation_reason: String,
    pub moderation_information: String,
    pub administrator: String,
    pub maintenance_lifetime: String,
    pub business_model: String,
    pub languages: Vec<String>,
    pub categories: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub instance: Instance,
    pub theme: custom::Theme,
    pub services: std::collections::HashMap<String, custom::Service>,
    pub cache: custom::Cache,
    pub signup: custom::Signup,
    pub admin: custom::Admin,
    pub contact_form: Enabled,
    pub user: User,
    pub transcoding: custom::Transcoding,
    pub live: custom::Live,
    pub import: custom::Import,
    pub trending: custom::Trending,
    pub auto_blacklist: AutoBlacklist,
    pub followers: custom::Followers,
    pub followings: custom::Followings,
    pub broadcast_message: BroadcastMessage,
    pub search: Search,
}
