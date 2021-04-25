use actix_web::{web, App, HttpServer};

mod routes;
use routes::crates_routes::{create_crate, delete_crate};
//
mod modules;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().service(
      web::scope("/crates")
        .service(create_crate)
        .service(delete_crate),
    )
  })
  .bind("127.0.0.1:3334")?
  .run()
  .await
}

//.service(
//     web::scope("/app1")
//     .data(State1)
//     .route("/", web::to(|| HttpResponse::Ok().body("web scope"))),
//     .route("/crates", web::get().to(create_crate)))
// // )
//
// .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
