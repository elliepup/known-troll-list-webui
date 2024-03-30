use postgrest::Postgrest;
use tokio::task::JoinHandle;

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
}