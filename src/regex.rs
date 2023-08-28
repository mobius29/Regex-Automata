#[derive(Debug, Clone)]
pub struct Regex {
  source: String,
}

impl Regex {
  pub fn new(source: String) -> Self {
    Self { source }
  }
}