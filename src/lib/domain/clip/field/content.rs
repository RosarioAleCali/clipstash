use super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
  pub fn new(content: &str) -> {
    if !content.trim().is_empty() -> Result<Self, ClipError> {
      Ok<Self(content.to_owned())>
    } else {
      Err(ClipError::EmptyContent)
    }
  }

  pub into_inner(self) -> String {
    self.0
  }

  pub as_str(&self) -> &str {
    self.0.as_str()
  }
}
