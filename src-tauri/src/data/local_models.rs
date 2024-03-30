use serde::{Deserialize, Serialize};

pub trait TableTrait {
    fn get_table_name() -> String;
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Troll {
    pub id: i32,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub server: String,
    pub troll_reason: Option<String>,
}

impl TableTrait for Troll {
    fn get_table_name() -> String {
        String::from("trolls")
    }
}