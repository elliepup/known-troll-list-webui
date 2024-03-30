use tokio::runtime::Runtime;

pub struct App{
    rt : Runtime
}


impl App{
    pub fn new() -> App{
        let rt = Runtime::new()
            .expect("Failed to create Tokio runtime");
        App{
            rt
        }
    }
}