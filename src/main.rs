use dotenv::dotenv;
mod db;
mod public;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // write the contest code here
    const CONTEST_CODE: &str = "B4E22F";
    let allusers_participated =
        services::allusers_participated::allusers_participated(CONTEST_CODE).await;
    match allusers_participated.len() {
        0 => println!("No users participated in this contest"),
        _ => {
            let users = allusers_participated[0]
                .get("users")
                .unwrap()
                .as_array()
                .unwrap();
            match public::table::table(&users) {
                Ok(_) => println!("Successfully created table.docx"),
                Err(e) => panic!("Failed to create table.docx: {:?}", e),
            }
        }
    }
}
