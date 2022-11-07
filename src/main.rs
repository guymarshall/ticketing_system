#![forbid(unsafe_code)]

mod page;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    let hello_page: page::Page = page::Page {title: "Hello Page".to_string(), content: "<p>This is some text! And <b>this</b> word is bold!</p><br><button class=\"btn btn-primary\" onclick=\"alert('This button was clicked!');\">Click me!</button>".to_string(), script_js: "".to_string()};
    HttpResponse::Ok().body(hello_page.create_page())
}

pub async fn add_ticket() -> impl Responder {
    let html = "
        <form method=\"post\">
            <label for=\"title\">Title: </label>
            <input type=\"text\" id=\"title\" name=\"title\">
            <br>
            <input type=\"submit\" id=\"save\" name=\"save\">
        </form>
    ";
    let js = "
        (\"#save\").click(() => {
            alert(\"Saved!\");
        });
    ";

    let add_ticket_page: page::Page = page::Page {title: "Add Ticket".to_string(), content: html.to_string(), script_js: js.to_string()};
    HttpResponse::Ok().body(add_ticket_page.create_page())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/tickets", web::get().to(hello))
            .route("/add_ticket", web::get().to(add_ticket))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}