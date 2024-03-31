use crate::data::local_models::TableTrait;
use serde::de::DeserializeOwned;

pub struct RedisManager {
    client: redis::Client
}

impl RedisManager {
    pub fn new() -> Result<RedisManager, String> {
        let client = redis::Client::open("redis://127.0.0.1/")
            .map_err(|e| e.to_string())?;
        Ok(RedisManager { client })
    }

    pub fn set_up_table<T: TableTrait + DeserializeOwned>(&self) -> Result<(), String> {
        let mut con = self.client.get_connection()
            .map_err(|e| e.to_string())?;

        let table_name = T::get_table_name();
        todo!();
    }
}