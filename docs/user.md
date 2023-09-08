# User

The user endpoint allows you to retrieve information about the user associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/users)

## Retrieve User

```rust
use lemonsqueezy::user::User;

// lemonsqueezy is an instance of the struct LemonSqueezy
let user = User::build(lemonsqueezy);
let user = user.retrieve().await.unwrap();
```

## Quick Links 
- [Back: Installation](index.md)
- [Next: Stores](stores.md)