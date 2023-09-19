# Installation 

## Requirements
- Rust 1.51.0 or higher
- Cargo 1.51.0 or higher

## Installation

### Using CLI
```bash
cargo add lemonsqueezy
```

### Using Cargo.toml
```toml
[dependencies]
lemonsqueezy = "0.1.0"
```

## Usage
```rust
use lemonsqueezy::user::User;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Create a new instance of LemonSqueezy with your API key
    let lemonsqueezy = lemonsqueezy::LemonSqueezy::new(std::env::var("API_KEY").unwrap());

    // Build a new instance of `User`
    let user = User::build(lemonsqueezy);
    
    // Retrieve the user
    let user = user.retrieve().await.unwrap();

    println!("{:#?}", user);
}
```

## Quick Links 
- [Next: User](user.md)