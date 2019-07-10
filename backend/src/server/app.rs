use crate::ase;
use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn get_server_list(_req: HttpRequest) -> impl Responder {
    web::Json(ase::list::get().unwrap())
}

#[rustfmt::skip]
pub fn run(addr: String) {
    HttpServer::new(|| {
        let routes =
            web::scope("/api")
                .service(web::scope("/list")
                    .route("", web::get().to(get_server_list)));

        let cors = Cors::new().allowed_origin("http://localhost:8080");

        App::new()
            .wrap(cors)
            .service(routes)
    })
    .bind(&addr)
    .expect(&format!("can't bind to {}", &addr))
    .run()
    .unwrap()
}
