use std::collections::HashMap;
use tokio::runtime::Runtime;
use crate::utilities::data_manager::{DbConnectionArgs, DbManager};
use crate::utilities::file_manager::get_env_from_config;
use crate::data::local_models::{Comment, Troll};

pub struct App {
    db_manager: DbManager,
    rt: Runtime,
    trolls: HashMap<i32, Troll>,
    comments: HashMap<i32, Vec<Comment>>,
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

        let comments = rt.block_on(async { db_manager.get_table::<Comment>().await})
            .map_err(|e| e.to_string())?
            .into_iter()
            .fold(HashMap::new(), |mut map, comment| {
                map.entry(comment.troll_id)
                    .or_insert_with(Vec::new)
                    .push(comment);
                map
            });

        Ok(App{
            db_manager,
            rt,
            trolls,
            comments
        })
    }

    pub fn initialize(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn get_troll_by_id<T: Into<i32>>(&self, id: T) -> Option<&Troll> {
        self.trolls.get(&id.into())
    }

    pub fn get_trolls_by_name(&self, name: &str) -> Vec<&Troll> {
        let lowercase_name = name.to_lowercase();
        self.trolls.values()
            .filter(|troll| troll.first_name.to_lowercase().contains(&lowercase_name)
                || troll.last_name.to_lowercase().contains(&lowercase_name))
            .collect()
    }

    pub fn get_troll_comments<T: Into<i32>>(&self, id: T) -> Option<Vec<&Comment>>
    {
        self.comments.get(&id.into())
            .map(|comments| comments.iter().collect())
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