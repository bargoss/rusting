#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

//username:
//asdasdajklj232
//
//pass:
//asdasd97gf8asdad


//use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
//use std::env;
//use std::error::Error;
//use tokio;
//#[tokio::main]
//async fn mongo_example() -> Result<(), Box<dyn Error>> {
//    // Load the MongoDB connection string from an environment variable:
//    let client_uri =
//        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
//
//    // A Client is needed to connect to MongoDB:
//    // An extra line of code to work around a DNS issue on Windows:
//    let options =
//        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
//            .await?;
//    let client = Client::with_options(options)?;
//
//    // Print the databases in our MongoDB cluster:
//    println!("Databases:");
//    for name in client.list_database_names(None, None).await? {
//        println!("- {}", name);
//    }
//
//    Ok(())
//}