use tokio::runtime::Runtime;
use crate::utilities::data_manager::{DbConnectionArgs, DbManager};
use crate::utilities::file_manager::get_env_from_config;

pub struct App {
    db_manager: DbManager,
    rt : Runtime
}


impl App{
    pub fn new() -> Result<App, String>{
        let rt = Runtime::new()
            .map_err(|e| e.to_string())?;

        let section = "DBSettings";
        let url = get_env_from_config(section, "known_troll_db_url")?;
        let key = get_env_from_config(section, "known_troll_db_api_key")?;

        let args = DbConnectionArgs{
            db_url: url,
            db_api_key: key
        };

        //let db_manager = rt
        //    .block_on(DbManager::new(args))
        //    .expect("Failed to connect to the database");

        let db_manager = DbManager::new(args);

        Ok(App{
            db_manager,
            rt
        })
    }
}