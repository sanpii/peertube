#[derive(Debug, serde::Deserialize)]
pub struct Cache {
    previews: Size,
    captions: Size,
    torrents: Size,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Size {
    pub size: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Theme {
    pub default: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Service {
    pub username: String,
    pub whitelisted: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signup {
    pub enabled: bool,
    pub limit: u32,
    pub requires_email_verification: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct Admin {
    pub email: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transcoding {
    pub enabled: bool,
    pub allow_additional_extensions: bool,
    pub allow_audio_files: bool,
    pub threads: u32,
    pub concurrency: u32,
    pub profile: String,
    pub resolutions: std::collections::HashMap<String, bool>,
    pub webtorrent: super::Enabled,
    pub hls: super::Enabled,
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
}

#[derive(Debug, serde::Deserialize)]
pub struct LiveTranscoding {
    pub enabled: bool,
    pub threads: u32,
    pub profile: String,
    pub resolutions: std::collections::HashMap<String, bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Import {
    pub videos: Videos,
}

#[derive(Debug, serde::Deserialize)]
pub struct Videos {
    pub concurrency: u32,
    pub http: super::Enabled,
    pub torrent: super::Enabled,
}

#[derive(Debug, serde::Deserialize)]
pub struct Trending {
    pub videos: TrendingVideo,
}

#[derive(Debug, serde::Deserialize)]
pub struct TrendingVideo {
    pub algorithms: super::Algorithms,
}

#[derive(Debug, serde::Deserialize)]
pub struct Followers {
    pub instance: FollowersInstance,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowersInstance {
    pub enabled: bool,
    pub manual_approval: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct Followings {
    pub instance: FollowingsInstance,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowingsInstance {
    pub auto_follow_back: super::Enabled,
    pub auto_follow_index: AutoFollowIndex,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoFollowIndex {
    pub enabled: bool,
    pub index_url: String,
}
