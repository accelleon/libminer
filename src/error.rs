use reqwest::Error as ReqwestError;
use thiserror::Error;
use std::io::Error as IoError;
use serde_json::Error as JsonError;
use digest_auth::Error as DigestAuthError;
use reqwest::header::ToStrError;
use crate::miners::avalon::DeError as AvalonDeError;

#[derive(Error, Debug)]
pub enum Error {
    // Errors bubbled from dependencies
    #[error("Reqwest error")]
    RequestError(#[from] ReqwestError),
    #[error("Io error")]
    IoError(#[from] IoError),
    #[error("Json error")]
    ParseError(#[from] JsonError),
    #[error("Digest auth error")]
    DigestAuthError(#[from] DigestAuthError),
    #[error("ToStr error")]
    ToStrError(#[from] ToStrError),
    #[error("Failed to acquire semaphore")]
    SemaphoreError(#[from] tokio::sync::AcquireError),

    #[cfg(feature = "avalon")]
    #[error("Avalon deserializer error")]
    AvalonDeserializerError(#[from] AvalonDeError),

    // Errors from this library
    // Detection errors
    #[error("No host detected")]
    NoHostDetected,
    #[error("Unable to detect miner type")]
    UnknownMinerType,
    
    // Response parsing errors
    #[error("Encode error")]
    EncodingError,

    // Network errors
    #[error("Timeout")]
    Timeout,
    #[error("Connection refused")]
    ConnectionRefused,
    #[error("Failed to execute HTTP request")]
    HttpRequestFailed,

    // API errors
    #[error("Token expired")]
    TokenExpired,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("API Call failed: {0}")]
    ApiCallFailed(String),
    #[error("Expected return")]
    ExpectedReturn,
    #[error("Not supported")]
    NotSupported,
    #[error("Invalid response")]
    InvalidResponse,
}
