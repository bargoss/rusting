// use tokio create simple async main that prints hello

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    sleep(Duration::from_secs(1)).await;
    println!("Hello, world!");
}




