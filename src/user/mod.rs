use reqwest::Error;

use crate::EtsyClient;

pub async fn me(client: &EtsyClient) -> Result<(), Error> {
    let response = client.get("users/me").await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}

pub async fn get(client: &EtsyClient, user_id: i32) -> Result<(), Error> {
    let response = client.get(&format!("users/{}", user_id)).await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}

