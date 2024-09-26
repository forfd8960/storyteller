use anyhow::Result;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};

pub mod errors;
pub mod models;
pub mod server;
pub mod service;

pub struct AppConfig {
    port: u8,
    db_url: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pool: PgPool,
}

impl AppState {
    pub async fn new(conf: &AppConfig) -> Result<Self> {
        let pool = PgPool::connect(&conf.db_url).await?;
        Ok(Self {
            inner: Arc::new(AppStateInner { pool }),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
