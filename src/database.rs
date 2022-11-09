pub struct Database {
    pub db_username: String,
    pub db_password: String,
    pub db_host: String
}
impl Database {
    pub fn insert(table: String, data: String) -> String {
        // insert data to table
        // return response as String
    }

    pub fn select(table: String) -> String {
        // select data from table
        // return response as String
    }

    pub fn update(table: String, data: String) -> String {
        // update data in table
        // return response as String
    }

    pub fn delete(table: String, id: i32) -> String {
        // delete row in table associated with id
        // return response as String
    }

    pub fn execute_query(sql: String) -> String {
        // execute query
        // return response as String
    }
}