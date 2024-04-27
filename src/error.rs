use thiserror::Error;
use url::ParseError;

use crate::jwt;

#[derive(Debug, Error)]
pub enum ParamError {
  #[error("Invalid params for {msg}")]
  InvalidParams { msg: String },
}

#[derive(Debug, Error)]
pub enum ClientError {
  #[error(transparent)]
  /// Thrown when JWT signing fails
  JwtError(#[from] jwt::JwtError),

  #[error(transparent)]
  /// Thrown when Token fails
  TokenError(#[from] jsonwebtoken::errors::Error),

  #[error(transparent)]
  /// Thrown when submitting a POST/GET request fails
  ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum FireblocksError {
  #[error(transparent)]
  /// Thrown when JWT signing fails
  JwtError(#[from] jwt::JwtError),

  #[error("Deserialization Error: {err}. Response: {text}")]
  /// Serde JSON Error
  SerdeJson { request_id: String, err: serde_json::Error, text: String },

  #[error(transparent)]
  /// Thrown when submitting a POST/GET request fails
  ReqwestError(#[from] reqwest::Error),

  #[error(transparent)]
  UrlError(#[from] ParseError),

  #[error(transparent)]
  QueryParamError(#[from] ParamError),

  #[error("Internal Fireblocks Error. HTTP Code {code} {text} request_id:{request_id}")]
  InternalError { request_id: String, path: String, code: u16, text: String },

  #[error("{path} not found. request_id: {request_id}")]
  NotFound { request_id: String, path: String },

  #[error("Bad Request for {path} {text} request_id: {request_id}")]
  BadRequest { request_id: String, path: String, text: String },

  #[error("Unauthorized for {path} {text} request_id: {request_id}")]
  Unauthorized { request_id: String, path: String, text: String },

  #[error("Forbidden for {path} {text} request_id: {request_id}")]
  Forbidden { request_id: String, path: String, text: String },

  #[error("Unknown Error HTTP Code: {code} request_id: {request_id}")]
  Unknown { request_id: String, path: String, code: u16, text: String },

  #[error("Invalid Request Error: {text}. Code: {code} request_id: {request_id}")]
  InvalidRequest { request_id: String, code: u16, text: String },
}
