use crate::modules::crates::model::CrateModel;

use super::icrate_repository::{ICrateDTO, ICrateRepository};
pub struct CrateRepository;

impl ICrateRepository for CrateRepository {
  fn create(data: &ICrateDTO) -> CrateModel {
    CrateModel {
      id: String::from("id123"),
      name: data.name.clone(),
      url: data.url.clone(),
    }
  }
}
