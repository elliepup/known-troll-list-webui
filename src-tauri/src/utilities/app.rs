use tokio::runtime::Runtime;
use crate::utilities::data_manager::{DbConnectionArgs, DbManager};
use crate::utilities::redis_manager::RedisManager;
use crate::utilities::file_manager::get_env_from_config;
use crate::data::local_models::{Troll, Comment};

pub struct App {
    db_manager: DbManager,
    redis_manager: RedisManager,
    rt: Runtime
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

        let redis_manager = RedisManager::new()
            .map_err(|e| e.to_string())?;

        Ok(App{
            db_manager,
            redis_manager,
            rt
        })
    }

    pub fn initialize(&self) -> Result<(), String> {
        self.init_redis()?;
        Ok(())
    }

    fn init_redis(&self) -> Result<(), String> {
        self.redis_manager.set_up_table::<Troll>()?;
        self.redis_manager.set_up_table::<Comment>()?;
        Ok(())
    }

    pub fn get_trolls(&self) -> Option<Vec<String>> {
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