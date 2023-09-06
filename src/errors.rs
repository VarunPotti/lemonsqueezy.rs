use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Failed to make request: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Status code: {0}")]
    StatusCodeError(reqwest::StatusCode),
}
