use std::fs;

pub struct Page {
    pub title: String,
    pub content: String,
    pub script_js: String
}
impl Page {
    pub fn create_page(&self) -> String {
        let title: &String = &self.title;
        let content: &String = &self.content;
        let script_js: &String = &self.script_js;
        let app_name: String = "Ticketing System".to_string();

        let my_css: String = fs::read_to_string("static/css/styles.css").expect("Unable to read file");
        let bootstrap_css: String = fs::read_to_string("static/css/bootstrap.css").expect("Unable to read file");
        let bootstrap_js: String = fs::read_to_string("static/js/bootstrap.bundle.js").expect("Unable to read file");
        let jquery_js: String = fs::read_to_string("static/js/jquery-3.6.1.js").expect("Unable to read file");

        let head_html: String = format!{"
            <meta charset=\"UTF-8\">
            <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <style>{my_css}</style>
            <style>{bootstrap_css}</style>
            <script>{bootstrap_js}</script>
            <script>{jquery_js}</script>
        "};

        let body_html: String = format!{"
            <nav class=\"navbar navbar-expand-sm navbar-toggleable-sm navbar-light border-bottom mb-3\">
                <div class=\"container-fluid\">
                    <a class=\"navbar-brand\" href=\"./all\">{app_name}</a>
                    <button class=\"navbar-toggler\" type=\"button\" data-bs-toggle=\"collapse\" data-bs-target=\".navbar-collapse\" aria-controls=\"navbarSupportedContent\" aria-expanded=\"false\" aria-label=\"Toggle navigation\">
                        <span class=\"navbar-toggler-icon\"></span>
                    </button>
                    <div class=\"navbar-collapse collapse d-sm-inline-flex justify-content-between\">
                        <ul class=\"navbar-nav flex-grow-1\">
                            <li class=\"nav-item\">
                                <a class=\"nav-link {}\" href=\"/all\">All</a>
                            </li>
                            <li class=\"nav-item\">
                                <a class=\"nav-link {}\" href=\"/new\">New</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
            <div class=\"container\">
                <main role=\"main\" class=\"pb-3\">
                    {content}
                </main>
            </div>
        ",
        (if title == "All" {"active"} else {""}),
        (if title == "New" {"active"} else {""})};

        let template_html: String = format!{"
            <!DOCTYPE html>
            <html lang=\"en\">
            <head>
                {head_html}
                <title>{title}</title>
            </head>
            <body>
                {body_html}
                <script type=\"text/javascript\">{script_js}</script>
            </body>
            </html>
        "};

        template_html
    }
}