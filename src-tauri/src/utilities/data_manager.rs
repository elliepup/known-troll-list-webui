use postgrest::Postgrest;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::task::JoinHandle;
use crate::data::local_models::TableTrait;

pub struct DbManager{
    client: Postgrest
}

pub struct DbConnectionArgs{
    pub db_url: String,
    pub db_api_key: String
}

impl DbManager {
    pub fn new(args: DbConnectionArgs) -> JoinHandle<DbManager> {
        tokio::spawn(async move {
            let client = Postgrest::new(args.db_url)
                .insert_header("apikey", args.db_api_key);
            println!("Connected to database");
            DbManager { client }
        })
    }

    pub async fn get_table<T: DeserializeOwned + TableTrait>(&self) -> Result<Vec<T>, String> {
        let table_name = T::get_table_name();

        let resp = self.client
            .from(table_name).select("*")
            .execute().await
            .map_err(|e| e.to_string())?;

        resp.text().await.map_err(|e| e.to_string())
            .and_then(|text| serde_json::from_str(&text)
                .map_err(|e| e.to_string()))
    }

    pub async fn database_insert<T: Serialize + TableTrait>(&self, obj: T) -> Result<(), String> {
        let table_name = T::get_table_name();
        let body = serde_json::to_string(&obj).map_err(|e| e.to_string())?;

        let response = self.client
            .from(table_name)
            .insert(body)
            .execute().await
            .map_err(|e| e.to_string())?;

        response.status()
            .is_success()
            .then_some(())
            .ok_or_else(|| "Failed to insert".to_string())
    }
}