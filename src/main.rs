mod page;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    let hello_page: page::Page = page::Page {title: "Hello Page".to_string(), content: "<p>This is some text! And <b>this</b> word is bold!</p><br><button class=\"btn btn-primary\" onclick=\"alert('This button was clicked!');\">Click me!</button>".to_string(), script_js: "".to_string()};
    HttpResponse::Ok().body(hello_page.create_page())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}