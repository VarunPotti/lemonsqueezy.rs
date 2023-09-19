# Customer

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/stores)

## Retrieve Customer

```rust
use lemonsqueezy::customer::Customer;

let customer = Customer::build(lemonsqueezy);
let customer = customer.retrieve(123).await.unwrap();
```

## Get All Stores

```rust
use lemonsqueezy::customer::Customer;

let customer = Customer::build(lemonsqueezy);
let customer = customer.get_all().await.unwrap();
```

## Quick Links 
- [Back: Stores](stores.md)
- [Next: Products](products.md)