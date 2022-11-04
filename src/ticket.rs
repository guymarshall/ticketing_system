pub struct Ticket {
    pub ticket_id: i32,
    pub title: String,
    pub status: String
}
impl Ticket {
    pub fn get_ticket_id(&self) -> i32 {
        self.ticket_id
    }
    pub fn get_title(&self) -> String {
        self.title
    }
    pub fn get_status(&self) -> String {
        self.status
    }
}