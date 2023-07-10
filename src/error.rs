use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("{0}")]
  ReqwestError(#[from] reqwest::Error),
  #[error("Resend Error: {0}")]
  ResendError(String),
}
