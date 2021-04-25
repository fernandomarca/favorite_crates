use crate::modules::crates::{
  model::CrateModel,
  repositories::{
    crate_repository::CrateRepository,
    icrate_repository::{ICrateDTO, ICrateRepository},
  },
};

pub struct CreateCrateService;

impl CreateCrateService {
  pub fn execute(&self, data: ICrateDTO) -> CrateModel {
    //criar no repositório o crate crateRepository.create();
    let new_crate = CrateRepository::create(&data);
    new_crate
  }
}
