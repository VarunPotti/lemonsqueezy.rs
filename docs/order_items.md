# Order Items

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/order-items#the-order-item-object)

## Retrieve Order Items

```rust
use lemonsqueezy::order_items::OrderItem;

let order_items = OrderItem::build(lemonsqueezy);
let order_items = order_items.retrieve(123).await.unwrap();
```

## Get All Order Items

```rust
use lemonsqueezy::order_items::OrderItem;

let order_items = OrderItem::build(lemonsqueezy);
let order_items = order_items.get_all(None).await.unwrap();
```

## Quick Links 
- [Back: Variants](variants.md)
- [Next: Subscriptions](subscriptions.md)