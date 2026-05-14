use serde::Serialize;

#[derive(Serialize)]
pub struct Uri {
  pub uri: &'static str,
  pub template: &'static str,
}

impl Uri {
  pub const fn new(uri: &'static str, template: &'static str) -> Self {
    Self { uri, template }
  }
}
