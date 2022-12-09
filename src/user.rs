#![forbid(unsafe_code)]

pub struct User {
    pub user_id: i32,
    pub forename: String,
    pub surname: String,
    pub username: String,
    pub password: String
}
impl User {
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
    pub fn get_forename(&self) -> String {
        self.forename
    }
    pub fn get_surname(&self) -> String {
        self.surname
    }
    pub fn get_username(&self) -> String {
        self.username
    }
    pub fn get_password(&self) -> String {
        self.password
    }
}