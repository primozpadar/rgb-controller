use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "r:{:<3}  g:{:<3}  b:{:<3}", self.r, self.g, self.b)
  }
}
