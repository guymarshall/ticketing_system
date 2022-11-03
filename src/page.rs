pub struct Page<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub script_js: &'a str
}
impl Page<'_> {
    pub fn create_page(&self) -> String {
        let title: &str = self.title;
        let content: &str = self.content;
        let script_js: &str = self.script_js;
        let app_name: &str = "Ticketing System";

        let head_html: &str = "
            <meta charset=\"UTF-8\">
            <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css\" integrity=\"sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3\" crossorigin=\"anonymous\">
            <script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js\" integrity=\"sha384-ka7Sk0Gln4gmtz2MlQnikT1wXgYsOg+OMhuP+IlRH9sENBO0LRn5q+8nbTov4+1p\" crossorigin=\"anonymous\"></script>
            <script src=\"https://code.jquery.com/jquery-3.6.1.min.js\" integrity=\"sha256-o88AwQnZB+VDvE9tvIXrMQaPlFFSUTR+nldQm1LuPXQ=\" crossorigin=\"anonymous\"></script>
        ";

        let body_html: String = format!{"
            <nav class=\"navbar navbar-expand-sm navbar-toggleable-sm navbar-light border-bottom mb-3\">
                <div class=\"container-fluid\">
                    <a class=\"navbar-brand\" href=\"./index.php\">{}</a>
                    <button class=\"navbar-toggler\" type=\"button\" data-bs-toggle=\"collapse\" data-bs-target=\".navbar-collapse\" aria-controls=\"navbarSupportedContent\" aria-expanded=\"false\" aria-label=\"Toggle navigation\">
                        <span class=\"navbar-toggler-icon\"></span>
                    </button>
                    <div class=\"navbar-collapse collapse d-sm-inline-flex justify-content-between\">
                        <ul class=\"navbar-nav flex-grow-1\">
                            <li class=\"nav-item\">
                                <a class=\"nav-link {}\" href=\"/pages/index.php\">Home</a>
                            </li>
                            <li class=\"nav-item\">
                                <a class=\"nav-link {}\" href=\"/pages/todo.php\">Todo</a>
                            </li>
                            <li class=\"nav-item\">
                                <a class=\"nav-link {}\" href=\"/pages/collatz.php\">Collatz</a>
                            </li>
                            <li class=\"nav-item\">
                                <a class=\"nav-link {}\" href=\"/pages/factorial.php\">Factorial</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
            <div class=\"container\">
                <main role=\"main\" class=\"pb-3\">
                    {}
                </main>
            </div>
        ",
        app_name,
        (if title == "Home" {"active"} else {""}),
        (if title == "Todo" {"active"} else {""}),
        (if title == "Collatz" {"active"} else {""}),
        (if title == "Factorial" {"active"} else {""}),
        content};

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