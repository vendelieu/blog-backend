use chrono::NaiveDateTime;

use crate::validator::Validate;

#[derive(Debug, Deserialize)]
pub enum Sort {
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest
}

#[derive(Debug, Deserialize, Validate)]
pub struct PostFilter {
    pub title: Option<String>,
    pub keyword: Option<String>,
    #[validate(length(max = 20))]
    pub tags: Option<String>,
    pub slug: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub sort_by: Option<Sort>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CommentaryFilter {
    pub username: Option<String>,
    pub text: Option<String>,
    pub reply_to: Option<i32>,
    pub date: Option<NaiveDateTime>,
    pub sort_by: Option<Sort>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}