#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("'Invalid header: {0}")]
    InvalidHeader(String),
}