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
            <div class=\"form-group row\">
                <label for=\"title\" class=\"col-sm-2 control-label\">Title: </label>
                <div class=\"col-sm-4\">
                    <input class=\"form-control\" type=\"text\" name=\"title\" id=\"title\">
                </div>
                <label for=\"name\" class=\"col-sm-2 control-label\">Name: </label>
                <div class=\"col-sm-4\">
                    <input class=\"form-control\" type=\"text\" name=\"name\" id=\"name\">
                </div>
            </div>
            <div class=\"form-group row\">
                <label for=\"priority\" class=\"col-sm-2 control-label\">Priority: </label>
                <div class=\"col-sm-4\">
                    <select class=\"form-control\" name=\"priority\" id=\"priority\">
                        <option value=\"low\">Low</option>
                        <option value=\"medium\">Medium</option>
                        <option value=\"high\">High</option>
                        <option value=\"critical\">Critical</option>
                    </select>
                </div>
            </div>
            <div class=\"form-group row\">
                <label for=\"description\" class=\"col-sm-2 control-label\">Description: </label>
                <div class=\"col-sm-10\">
                    <textarea class=\"form-control\" name=\"description\" id=\"description\" rows=\"4\" style=\"overflow: auto; resize: none\"></textarea>
                </div>
            </div>
            <div class=\"form-group row\">
                <label for=\"comments\" class=\"col-sm-2 control-label\">Comments: </label>
                <div class=\"col-sm-10\">
                    <textarea class=\"form-control\" name=\"comments\" id=\"comments\" rows=\"4\" style=\"overflow: auto; resize: none\"></textarea>
                </div>
            </div>
            <button type=\"button\" class=\"btn btn-outline-primary\">Submit</button>
        </form>
    ";
    let js: &str = "";

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