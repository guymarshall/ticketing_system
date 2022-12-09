#![forbid(unsafe_code)]

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

fn init_database() {
    let user_table_sql: String = "
        CREATE TABLE users (
            userID INTEGER PRIMARY KEY NOT NULL,
            forename TEXT NOT NULL,
            surname TEXT NOT NULL,
            position TEXT NOT NULL,
            isDeleted BOOLEAN NOT NULL DEFAULT '0'
        )
    ".to_string();
    
    let ticket_table_sql: String = "
        CREATE TABLE tickets (
            ticketID INTEGER PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            comments TEXT NOT NULL,
            isDeleted BOOLEAN NOT NULL DEFAULT '0'
        )
    ".to_string();
}