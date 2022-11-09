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

        let head_html: String = String::from("
            <meta charset=\"UTF-8\">
            <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css\" integrity=\"sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3\" crossorigin=\"anonymous\">
            <script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js\" integrity=\"sha384-ka7Sk0Gln4gmtz2MlQnikT1wXgYsOg+OMhuP+IlRH9sENBO0LRn5q+8nbTov4+1p\" crossorigin=\"anonymous\"></script>
            <script src=\"https://code.jquery.com/jquery-3.6.1.min.js\" integrity=\"sha256-o88AwQnZB+VDvE9tvIXrMQaPlFFSUTR+nldQm1LuPXQ=\" crossorigin=\"anonymous\"></script>
        ");

        let body_html: String = format!{"
            <nav class=\"navbar navbar-expand-sm navbar-toggleable-sm navbar-light border-bottom mb-3\">
                <div class=\"container-fluid\">
                    <a class=\"navbar-brand\" href=\"./tickets\">{app_name}</a>
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
                            <li class=\"nav-item\">
                                <a class=\"nav-link {}\" href=\"/collatz.php\">Collatz</a>
                            </li>
                            <li class=\"nav-item\">
                                <a class=\"nav-link {}\" href=\"/factorial.php\">Factorial</a>
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
        (if title == "Home" {"active"} else {""}),
        (if title == "Todo" {"active"} else {""}),
        (if title == "Collatz" {"active"} else {""}),
        (if title == "Factorial" {"active"} else {""})};

        let template_html: String = format!{"
            <!DOCTYPE html>
            <html lang=\"en\">
            <head>
                {}
                <title>{}</title>
            </head>
            <body>
                {}
                <script type=\"text/javascript\">{}</script>
            </body>
            </html>
        ",
        head_html,
        title,
        body_html,
        script_js};

        template_html
    }
}