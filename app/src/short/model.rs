use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateShort {
  pub url: String,
}
