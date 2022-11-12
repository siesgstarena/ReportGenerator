use futures::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Collection,
};

use crate::db::connect::connect;
pub async fn allusers_participated(contest_id: &str) -> Vec<Document> {
    let c = connect().await;
    let submissions: Collection<Document> = c.collection("contests");
    let mut all_users = Vec::new();
    let pipe_line = vec![
        doc! {
            "$match":{
                "code":contest_id.to_string()
            }
        },
        doc! {
            "$lookup":{
                "from": "submissions",
                "localField": "_id",
                "foreignField": "contestId",
                "as": "submissions"
            }
        },
        doc! {
            "$match":{
                "submissions.duringContest":true
            }
        },
        doc! {
            "$project":{
                "userId":"$submissions.userId"
            }
        },
        doc! {
            "$unwind":{
                "path": "$userId",
                "preserveNullAndEmptyArrays": true
            }
        },
        doc! {
            "$lookup":{
                "from":"users",
                "localField":"userId",
                "foreignField":"_id",
                "as":"users"
            }
        },
        doc! {
            "$project":{
                "users":{
                    "$arrayElemAt":["$users",0]
                }
            }
        },
        doc! {
            "$project":{

                "name":"$users.name",
            }
        },
        doc! {
            "$unwind":{
                "path":"$name",
                "preserveNullAndEmptyArrays":true
            }
        },
        doc! {
            "$group":{
                "_id":"null",
                "users":{
                    "$addToSet":"$name"
                }
            }
        },
    ];
    let mut alluserscursor = submissions.aggregate(pipe_line, None).await.unwrap();
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
