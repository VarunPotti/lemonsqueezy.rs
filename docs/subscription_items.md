# Subscription Items

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/subscription-items#the-subscription-item-object)

## Retrieve Subscription Items

```rust
use lemonsqueezy::subscription_items::SubscriptionItems;

let subscription_items = SubscriptionItems::build(lemonsqueezy);
let subscription = subscription_items.retrieve(123).await.unwrap();
```

## Get All Subscription Items

```rust
use lemonsqueezy::subscription_items::SubscriptionItems;

let subscription_items = SubscriptionItems::build(lemonsqueezy);
let subscription_items = subscription_items.get_all(None).await.unwrap();
```

## Update a Subscription Item
```rust
use lemonsqueezy::subscription_items::{SubscriptionItems, SubscriptionItemPatchRequest};
use lemonsqueezy::utils::ResponseData;

let subscription_items = Subscription::build(lemonsqueezy);
let subscription = SubscriptionItemPatchRequest {
    quantity: Some(2),
    ..Default::default()
};
let subscription_items = subscription_items.update(ResponseData {
    r#type: "subscription-items",
    id: "123",// This is the subscription item id
    attributes: subscription
}).await.unwrap();
```

## Get Current Usage for a Subscription Item
```rust
use lemonsqueezy::subscription_items::SubscriptionItems;

let subscription_items = SubscriptionItems::build(lemonsqueezy);
let subscription = subscription_items.current_usage(123).await.unwrap();
```


## Quick Links 
- [Back: Subscription Invoices](subscription_invoice.md)
- [Next: Usage Records](usage_records.md)