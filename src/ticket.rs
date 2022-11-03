pub struct Ticket<'a> {
    pub id: i32,
    pub title: &'a str,
    pub status: &'a str
}
impl Ticket<'_> {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_title(&self) -> &str {
        self.title
    }
    pub fn get_status(&self) -> &str {
        self.status
    }
}