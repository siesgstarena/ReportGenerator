use mongodb::{options::ClientOptions, Client, Database};

pub async fn connect() -> Database {
    let client_options = ClientOptions::parse(std::env::var("DB_URI").expect("MAILCOACH_API_TOKEN must be set.")).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("arenabackup");
    db
}
