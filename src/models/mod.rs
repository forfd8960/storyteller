use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod story;
pub mod user;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct User {
    id: i64,
    username: String,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub(crate) struct CreateUser {
    username: String,
    email: String,
    password_hash: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Story {
    id: i64,
    title: String,
    author_id: i64,
    #[sqlx(default)]
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct WriteableStory {
    title: String,
    author_id: i64,
    content: String,
}

pub(crate) trait UserStorage {
    fn get_by_id(id: i64) -> Self;
    fn get_by_email(email: String) -> Self;
    fn create(user: &CreateUser) -> Self;
}

#[derive(Debug)]
pub struct ListStory {
    author_id: i64,
    key_word: String,
    order_by: String,
    asc: bool,
}

pub trait StoryStorage {
    fn get_by_id(id: i64) -> Result<Story>;
    fn list(list_req: &ListStory) -> Result<Vec<Story>>;
    fn create(story: &WriteableStory) -> Result<Story>;
    fn update(story: &Story) -> Result<Story>;
    fn delete(story_id: i64) -> Result<()>;
}

impl User {
    pub fn default() -> Self {
        let utc_now: DateTime<Utc> = Utc::now();
        Self { id: 0, username: "".to_string(), email: "".to_string(), password_hash: "".to_string(), created_at: utc_now }
    }
}