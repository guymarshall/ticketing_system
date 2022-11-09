#![forbid(unsafe_code)]

mod page;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn all() -> impl Responder {
    let title: &str = "All";
    let html: &str = "
        <p>This is some text! And <b>this</b> word is bold!</p>
        <br>
        <button class=\"btn btn-primary\" onclick=\"alert('This button was clicked!');\">Click me!</button>
    ";
    let js: &str = "";
    let all_page: page::Page = page::Page {title: title.to_string(), content: html.to_string(), script_js: js.to_string()};
    HttpResponse::Ok().body(all_page.create_page())
}

async fn add_ticket() -> impl Responder {
    let title: &str = "Add Ticket";
    let html: &str = "
        <form method=\"post\">
            <label for=\"title\">Title: </label>
            <input type=\"text\" id=\"title\" name=\"title\">
            <br>
            <input type=\"submit\" id=\"save\" name=\"save\">
        </form>
    ";
    let js: &str = "
        (\"#save\").click(() => {
            alert(\"Saved!\");
        });
    ";

    let add_ticket_page: page::Page = page::Page {title: title.to_string(), content: html.to_string(), script_js: js.to_string()};
    HttpResponse::Ok().body(add_ticket_page.create_page())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/tickets", web::get().to(all))
            .route("/add_ticket", web::get().to(add_ticket))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}