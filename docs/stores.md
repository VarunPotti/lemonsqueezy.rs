# Stores

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/stores)

## Retrieve Store

```rust
use lemonsqueezy::store::Store;

let store = Store::build(lemonsqueezy);
let store = store.retrieve(123).await.unwrap();
```

## Get All Stores

```rust
use lemonsqueezy::store::Store;

let store = Store::build(lemonsqueezy);
let store = store.get_all().await.unwrap();
```

## Quick Links 
- [Back: Installation](index.md)
- [Next: Customers](customers.md)