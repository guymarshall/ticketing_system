mod page;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// THIS NEEDS TO BE IN MAIN FILE - LOOK INTO
#[get("/anotherpage")]
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