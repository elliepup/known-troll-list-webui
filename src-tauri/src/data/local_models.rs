use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub trait TableTrait {
    fn get_table_name() -> String;
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Troll {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub first_name: String,
    pub last_name: String,
    pub server: String,
}
impl TableTrait for Troll {
    fn get_table_name() -> String {
        String::from("trolls")
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub created_at: String,
    pub comment: String,
    pub troll_id: i32,
}
impl TableTrait for Comment {
    fn get_table_name() -> String {
        String::from("comments")
    }
}