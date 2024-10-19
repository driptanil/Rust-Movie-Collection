use shared::models::version::Version;

pub type VersionError = String;
pub type VersionResult<T> = Result<T, VersionError>;

#[async_trait::async_trait]
pub trait VersionRepository: Send + Sync + 'static {
    async fn get_version(&self) -> VersionResult<Version>;
}
