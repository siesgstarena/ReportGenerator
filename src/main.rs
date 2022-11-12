use dotenv::dotenv;
mod public;
mod services;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let allusers_participated = services::allusers_participated::allusers_participated("B4E22Q").await;
    let users = allusers_participated[0].get("users").unwrap().as_array().unwrap();
    public::table::table(users).expect("Error in creating table");
    println!("Hello, world!");
}
