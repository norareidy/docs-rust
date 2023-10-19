use mongodb::{ 
    bson::doc,
    Client,
    Collection 
};
use serde::{ Deserialize, Serialize };
use futures::TryStreamExt;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    address: Address,
    borough: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter =
        doc! { "$and": [
           doc! { "borough": "Manhattan" },
           doc! { "address.street": "Broadway" }
       ]
    };

    let result = my_coll.delete_many(filter, None).await?;

    println!("Deleted documents: {}", result.deleted_count);

    Ok(())
}