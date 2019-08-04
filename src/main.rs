use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

fn main() {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| App::new().route("/", web::get().to(index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("localhost:5000").unwrap()
    };

    server.run().unwrap();
}
