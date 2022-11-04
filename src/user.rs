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
    pub fn get_forename(&self) -> &str {
        self.forename
    }
    pub fn get_surname(&self) -> &str {
        self.surname
    }
    pub fn get_username(&self) -> &str {
        self.username
    }
    pub fn get_password(&self) -> &str {
        self.password
    }
}