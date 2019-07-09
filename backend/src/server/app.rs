use crate::ase;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    format!("{:?}", ase::list::get().unwrap())
}

pub fn run(addr: String) {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(&addr)
        .expect(&format!("can't bind to {}", &addr))
        .run()
        .unwrap()
}
