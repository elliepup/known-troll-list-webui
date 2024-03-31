use std::collections::HashMap;
use tokio::runtime::Runtime;
use crate::utilities::data_manager::{DbConnectionArgs, DbManager};
use crate::utilities::file_manager::get_env_from_config;
use crate::data::local_models::{Troll, Comment};

pub struct App {
    db_manager: DbManager,
    rt: Runtime,
    trolls: HashMap<i32, Troll>,
}


impl App{
    pub fn new() -> Result<App, String> {
        let rt = Runtime::new()
            .map_err(|e| e.to_string())?;

        let section = "DBSettings";
        let db_url = get_env_from_config(section, "api_url_env")?;
        let db_api_key = get_env_from_config(section, "api_key_env")?;

        let args = DbConnectionArgs{db_url, db_api_key};

        let db_manager = rt.block_on(async { DbManager::new(args).await})
            .map_err(|e| e.to_string())?;

        let trolls = rt.block_on(async { db_manager.get_table::<Troll>().await})
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|troll| (troll.id, troll))
            .collect();

        Ok(App{
            db_manager,
            rt,
            trolls,
        })
    }

    pub fn initialize(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn get_troll_by_id<T: Into<i32>>(&self, id: T) -> Option<&Troll> {
        self.trolls.get(&id.into())
    }

    fn get_trolls(&self) -> Option<Vec<String>> { // Should only be used for testing
        let trolls = self.rt
            .block_on(async { self.db_manager.get_table::<Troll>().await})
            .ok()?;
        Some(trolls.iter()
            .map(|troll| format!("{} {}", troll.first_name, troll.last_name))
            .collect())
    }

    pub fn add_troll(&self, troll: Troll) -> Result<(), String> {
        self.rt.block_on(async { self.db_manager.database_insert(troll).await})
    }
}