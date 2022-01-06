use crate::domain::Time;
use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
  pub into_inner(self) -> Time {
    self.0
  }
}
