use etsy::{other::ping, user, EtsyClient};

#[tokio::main]
async fn main() {
    let client = EtsyClient::new("api_token");
    let _ = ping::ping(&client).await;
    let _ = user::me(&client).await;
    let _ = user::get(&client, 100).await;
    println!("Welcome to the CLI");
}
