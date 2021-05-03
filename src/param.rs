#[derive(Debug, Default, serde::Serialize)]
pub struct Accounts {
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
    pub count: Option<usize>,
    pub sort: Option<String>,
    pub start: Option<usize>,
    pub with_stats: Option<bool>,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    pub count: Option<usize>,
    pub rating: Option<Rating>,
    pub sort: Option<String>,
    pub start: Option<usize>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    Like,
    Dislike,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct Auth {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub response_type: String,
    pub username: String,
    pub password: String,
}
