use shared::models::version::Version;

use crate::repositories::{ version_repository::VersionResult, Repository };

#[async_trait::async_trait]
pub trait VersionService: Send + Sync {
    async fn get_version(&self) -> VersionResult<Version>;
}

pub struct VersionServiceImpl {
    pub repo: Repository,
}

impl VersionServiceImpl {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl VersionService for VersionServiceImpl {
    async fn get_version(&self) -> VersionResult<Version> {
        let result = self.repo.get_db_version().await;

        match result {
            Ok(db_version) => Ok(Version { db: db_version }),
            Err(e) => Err(e),
        }
    }
}
