pub struct User<'a> {
    pub user_id: i32,
    pub forename: &'a str,
    pub surname: &'a str,
    pub username: &'a str,
    pub password: &'a str
}
impl User<'_> {
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