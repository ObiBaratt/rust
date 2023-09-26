use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder, HttpResponse, post};
mod models;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let uppercase = req_body.to_uppercase();
    let lowercase = req_body.to_lowercase();
    let echoed = format!("{}...", req_body);
    let response = models::EchoResponse {
        uppercase,
        lowercase,
        echoed
    };
    HttpResponse::Ok().json(response)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8080;
    println!("Starting server on {}", &PORT);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}