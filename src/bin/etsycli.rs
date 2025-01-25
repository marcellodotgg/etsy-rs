use etsy::{other, user, EtsyClient};

#[tokio::main]
async fn main() {
    let client = EtsyClient::new("api_token");
    let _ = other::ping(&client).await;
    let _ = user::me(&client).await;
    let _ = user::get(&client, 100).await;
    println!("Welcome to the CLI");
}
