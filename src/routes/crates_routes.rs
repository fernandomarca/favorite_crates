use crate::modules::crates::repositories::icrate_repository::ICrateDTO;
use crate::modules::crates::services::create_crate_service::CreateCrateService;
use actix_web::{get, post, web, HttpResponse, Responder};
//route create service
#[post("/")]
pub async fn create_crate() -> impl Responder {
  let new_crate = CreateCrateService.execute(ICrateDTO {
    name: String::from("Node"),
    url: String::from("https://fmmagalhaes.com.br"),
  });

  web::Json(new_crate)
}

#[get("/")]
pub async fn delete_crate() -> impl Responder {
  HttpResponse::Ok().body("delete crate")
}

// #[route("/", method = "GET", method = "POST")]
// async fn example() -> impl Responder {
//   HttpResponse::Ok().finish()
// }
