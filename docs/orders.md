# Orders

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/orders#the-order-object)

## Retrieve Order

```rust
use lemonsqueezy::orders::Order;

let orders = Order::build(lemonsqueezy);
let orders = orders.retrieve(123).await.unwrap();
```

## Get All Orders

```rust
use lemonsqueezy::orders::Order;

let orderss = Order::build(lemonsqueezy);
let orderss = orderss.get_all(None).await.unwrap();
```

## Quick Links 
- [Back: Files](files.md)
- [Next: Order Items](order_items.md)