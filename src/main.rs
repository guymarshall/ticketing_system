use crate::ticket::Ticket;

mod ticket;

fn main() {
    let new_ticket: Ticket = Ticket { ticket_id: 1, title: "A test ticket", status: "Closed" };

    println!("ID: {}", new_ticket.get_ticket_id());
    println!("Title: {}", new_ticket.get_title());
    println!("Status: {}", new_ticket.get_status());
}
