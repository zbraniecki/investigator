use actix_cors::Cors;
use actix_web::{web, Route};
use actix_web::{App, HttpServer};
use investigator_server::{portfolio, server};

pub fn get_views() -> Vec<(&'static str, Route)> {
    vec![
        ("/portfolio/filter", web::get().to(portfolio::rest::filter)),
        ("/portfolio/create", web::get().to(portfolio::rest::create)),
        ("/portfolio/delete", web::get().to(portfolio::rest::delete)),
    ]
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = server::State::new().await;
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1234")
            .allowed_origin("http://127.0.0.1:1234")
            .supports_credentials();
        let mut app = App::new().wrap(cors).data(state.clone());
        for (path, view) in get_views() {
            app = app.route(path, view)
        }
        app
    })
    .bind("127.0.0.1:8080")?
    .run();
    server.await
}
