use crate::ase;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};

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

        App::new()
            .wrap(middleware::Logger::default())
            .service(routes)
    })
    .bind(&addr)
    .expect(&format!("can't bind to {}", &addr))
    .run()
    .unwrap()
}
