# Etsy API
This is a wrapper for the Etsy API. We are currently in the beta stages, this is not production ready and does not support any endpoints yet. Mostly using this project to learn and improve my Rust skills. Open to contributions.

## Roadmap

- [ ] Other
  - [ ] `ping`
- [ ] User
  - [ ] `getMe`
  - [ ] `getUser`

## Usage

```rs
use etsy::EtsyClient;
use etsy::{other, user};

#[tokio::main]
async fn main() {
  let client = EtsyClient::new("api_token");
  let _ = other::ping(&client).await; // will print the response  
  let _ = user::me(&client).await; // will print the response
  let _ = user::get(&client, 100).await; // will print the response
}

```
