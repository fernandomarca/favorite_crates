use serde::Serialize;
#[derive(Serialize)]
pub struct CrateModel {
  pub id: String,
  pub name: String,
  pub url: String,
}
