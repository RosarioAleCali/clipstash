use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{HitCounter, PASSWORD_COOKIE};
use crate::ServiceError;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::Responder;
use rocket::State;
use serde::Serialize;
use std::str::FromStr;

pub const API_KEY_HEADER: &str = "x-api-key";

#[derive(Responder, Debug, thiserror::Error, Serialize)]
pub enum ApiKeyError {
  #[error("API Key Not Found!")]
  #[response(status = 404, content_type = "json")]
  NotFound(String),

  #[error("Invalid API Key Format!")]
  #[response(status = 400, content_type = "json")]
  DecodeError(String),
}

#[derive(Debug, Clone)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
  pub fn to_base64(&self) -> String {
    base64::encode(self.0.as_slice())
  }

  pub fn into_inner(self) -> Vec<u8> {
    self.0
  }
}

impl Default for ApiKey {
  fn default() -> Self {
    let key = (0..16).map(|_| rand::random::<u8>()).collect();
    Self(key)
  }
}

impl FromStr for ApiKey {
  type Err = ApiKeyError;
  fn from_str(key: &str) -> Result<Self, Self::Err> {
    base64::decode(key)
      .map(ApiKey)
      .map_err(|e| Self::Err::DecodeError(e.to_string()))
  }
}

#[derive(Responder, Debug, thiserror::Error)]
pub enum ApiError {
  #[error("Not Found!")]
  #[response(status = 404, content_type = "json")]
  NotFound(Json<String>),

  #[error("Server Error!")]
  #[response(status = 500, content_type = "json")]
  Server(Json<String>),

  #[error("Client Error!")]
  #[response(status = 401, content_type = "json")]
  User(Json<String>),

  #[error("Key Error!")]
  #[response(status = 400, content_type = "json")]
  KeyError(Json<ApiKeyError>),
}

impl From<ServiceError> for ApiError {
  fn from(err: ServiceError) -> Self {
    match err {
      ServiceError::Clip(c) => Self::User(Json(format!("Clip parsing error: {}", c))),
      ServiceError::NotFound => Self::NotFound(Json("Entity Not Found".to_owned())),
      ServiceError::Data(_) => Self::Server(Json("A Server Error Occurred".to_owned())),
      ServiceError::PermissionError(msg) => Self::User(Json(msg)),
    }
  }
}
