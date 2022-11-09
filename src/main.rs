#![forbid(unsafe_code)]

mod page;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn root() -> impl Responder {
    let title: &str = "ACCESS DENIED";
    let html: &str = "
        <b style=\"color: red;\">DIRECTORY ACCESS IS FORBIDDEN</b>
    ";
    let js: &str = "";
    let root_page: page::Page = page::Page {title: title.to_string(), content: html.to_string(), script_js: js.to_string()};
    HttpResponse::Ok().body(root_page.create_page())
}

async fn all() -> impl Responder {
    let title: &str = "All";
    let html: &str = "
        <p>
            This is where a table of tickets will go.
        </p>
    ";
    let js: &str = "";
    let all_page: page::Page = page::Page {title: title.to_string(), content: html.to_string(), script_js: js.to_string()};
    HttpResponse::Ok().body(all_page.create_page())
}

async fn new() -> impl Responder {
    let title: &str = "New";
    let html: &str = "
        <form method=\"POST\">
            <label for=\"title\">Title: </label>
            <input type=\"text\" name=\"title\" id=\"title\">
            <br>
            <label for=\"username\">Username: </label>
            <input type=\"text\" name=\"username\" id=\"username\">
            <br>
            <label for=\"description\">Description: </label>
            <input type=\"text\" name=\"description\" id=\"description\">
            <br>
            <label for=\"priority\">Priority: </label>
            <select name=\"priority\" id=\"priority\">
                <option value=\"low\">Low</option>
                <option value=\"medium\">Medium</option>
                <option value=\"high\">High</option>
                <option value=\"critical\">Critical</option>
            </select>
            <br>
            <label for=\"comments\">Comments: </label>
            <input type=\"text\" name=\"comments\" id=\"comments\">
            <br>
            <input type=\"submit\" value=\"Submit\">
        </form>
    ";
    let js: &str = "
        (\"#save\").click(() => {
            alert(\"Saved!\");
        });
    ";

    let new_page: page::Page = page::Page {title: title.to_string(), content: html.to_string(), script_js: js.to_string()};
    HttpResponse::Ok().body(new_page.create_page())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .route("/all", web::get().to(all))
            .route("/new", web::get().to(new))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}