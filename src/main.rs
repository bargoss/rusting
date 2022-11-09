
/*
connect in javascript:
const { MongoClient, ServerApiVersion } = require('mongodb');
const uri = "mongodb+srv://asdasdajklj232:<password>@cluster0.c54afg2.mongodb.net/?retryWrites=true&w=majority";
const client = new MongoClient(uri, { useNewUrlParser: true, useUnifiedTopology: true, serverApi: ServerApiVersion.v1 });
client.connect(err => {
  const collection = client.db("test").collection("devices");
  // perform actions on the collection object
  client.close();
});
*/

//username:
//asdasdajklj232
//
//pass:
//asdasd97gf8asdad


use mongodb::{Client, Collection, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use mongodb::bson::Document;
use tokio;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    //let client_uri =
    //    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client_uri = "mongodb+srv://asdasdajklj232:asdasd97gf8asdad@cluster0.c54afg2.mongodb.net/?retryWrites=true&w=majority";

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;


    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }


    let coll: Collection<Document> = client.database("test").collection("devices");



    // print records
    let cursor = coll.find(None, None).await?;
    for result in cursor {
        if let Ok(item) = result {
            println!("Found document: {:?}", item);
        }
    }


    Ok(())
}