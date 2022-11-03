fn main() {
    struct Ticket<'a> {
        id: i32,
        title: &'a str,
        status: &'a str
    }
    impl Ticket<'_> {
        fn get_id(&self) -> i32 {
            self.id
        }
        fn get_title(&self) -> &str {
            self.title
        }
        fn get_status(&self) -> &str {
            self.status
        }
    }
    let new_ticket: Ticket = Ticket { id: 1, title: "A test ticket", status: "Closed" };

    println!("ID: {}", new_ticket.get_id());
    println!("Title: {}", new_ticket.get_title());
    println!("Status: {}", new_ticket.get_status());
}
