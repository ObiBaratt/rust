use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder, HttpResponse};
mod models;

#[get("/")]
async fn hello() -> HttpResponse {
    manual_hello().await
}

#[get("/echo")] 
async fn echo(req: web::Query<models::EchoRequest>) -> impl Responder {
    let message = req.message.clone().unwrap_or("".to_string());
    if message.len() == 0 {
        return HttpResponse::BadRequest().body("Bad request");
    }
    let uppercase = message.to_uppercase();
    let lowercase = message.to_lowercase();
    let echoed = format!("{}...", message);
    let response = models::EchoResponse {
        uppercase,
        lowercase,
        echoed
    };
    HttpResponse::Ok().json(response)
}

async fn manual_hello() -> HttpResponse {
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