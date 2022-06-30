use chrono::NaiveDateTime;
use crate::validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct PostFilter {
    pub title: Option<String>,
    pub keyword: Option<String>,
    #[validate(length(max = 20))]
    pub tags: Option<String>,
    pub slug: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CommentaryFilter {
    pub user_id: Option<i32>,
    pub text: Option<String>,
    pub reply_to: Option<i32>,
    pub date: Option<NaiveDateTime>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}