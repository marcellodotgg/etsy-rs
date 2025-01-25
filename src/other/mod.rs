use reqwest::Error;

use crate::EtsyClient;

/// Check to confirm connectivity to the Etsy API.
pub async fn ping(client: &EtsyClient) -> Result<(), Error> {
    let response = client.get("openapi-ping").await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
