use postgrest::Postgrest;

pub struct DbManager{
    client: Postgrest
}

pub struct DbConnectionArgs{
    pub db_url: String,
    pub db_api_key: String
}

impl DbManager{
    pub fn new(args: DbConnectionArgs) -> DbManager{
        let client = Postgrest::new(args.db_url)
            .insert_header("apikey", args.db_api_key);
        DbManager{ client }
    }
}