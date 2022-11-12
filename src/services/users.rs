use futures::StreamExt;
use mongodb::{bson::{ Document}};
use crate::db::connect::connect;
#[allow(dead_code)]
pub async fn users()-> Vec<Document> {
    let c  = connect().await;
    let users = c.collection("users");
    let mut all_users = Vec::new();
    let mut alluserscursor = users.find(None, None).await.unwrap();
    while let Some(result) = alluserscursor.next().await {
        match result {
            Ok(document) => {
                all_users.push(document);
            }
            Err(e) => panic!("Failed to fetch document: {:?}", e),
        }
    }
    all_users

}