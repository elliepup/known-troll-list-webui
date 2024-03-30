use tokio::runtime::Runtime;
use crate::utilities::data_manager::{DbConnectionArgs, DbManager};
use crate::utilities::file_manager::get_env_from_config;

pub struct App {
    db_manager: DbManager,
    _rt: Runtime
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

        Ok(App{
            db_manager,
            _rt: rt
        })
    }
}