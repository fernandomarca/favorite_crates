use crate::modules::crates::model::CrateModel;
use serde::Serialize;
#[derive(Serialize)]
pub struct ICrateDTO {
  pub name: String,
  pub url: String,
}

pub trait ICrateRepository {
  fn create(data: &ICrateDTO) -> CrateModel;
}
