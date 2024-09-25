use chrono::{DateTime, Utc};
use anyhow::Result;

pub mod story;
pub mod user;

#[derive(Debug)]
pub struct User {
    id: i64,
    username: String,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateUser {
    username: String,
    email: String,
    password_hash: String,
}

#[derive(Debug)]
pub struct Story {
    id: i64,
    title: String,
    author_id: i64,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct WriteableStory {
    title: String,
    author_id: i64,
    content: String,
}

pub trait UserStorage {
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
    fn update(story: &WriteableStory) -> Result<Story>;
    fn delete(story_id: i64) -> Result<()>;
}
