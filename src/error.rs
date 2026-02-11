#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("auth error: {0}")]
    Auth(#[from] google_cloud_auth::errors::CredentialsError),
    #[error("build auth error: {0}")]
    BuildAuth(#[from] google_cloud_auth::build_errors::Error),
    #[error("transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("status error: {0}")]
    Status(#[from] tonic::Status),
}
