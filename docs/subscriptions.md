# Subscriptions

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/subscriptions#the-subscription-object)

## Retrieve Subscriptions

```rust
use lemonsqueezy::subscriptions::Subscription;

let subscriptions = Subscription::build(lemonsqueezy);
let subscription = subscriptions.retrieve(123).await.unwrap();
```

## Get All Subscriptions

```rust
use lemonsqueezy::subscriptions::Subscription;

let subscriptions = Subscription::build(lemonsqueezy);
let subscriptions = subscriptions.get_all(None).await.unwrap();
```

## Update a Subscription
```rust
use lemonsqueezy::subscriptions::{Subscription,SubscriptionPatchRequest};
use lemonsqueezy::utils::ResponseData;

let subscriptions = Subscription::build(lemonsqueezy);
let subscription = SubscriptionPatchRequest {
    id: 123,
    status: Some("active".to_string()),
    ..Default::default()
};
let subscriptions = subscriptions.update(ResponseData {
    r#type: "subscriptions",
    id: "123",// This is the subscription id
    attributes: subscription
}).await.unwrap();
```

## Cancel a Subscription

```rust
use lemonsqueezy::subscriptions::Subscription;

let subscriptions = Subscription::build(lemonsqueezy);
let subscription = subscriptions.cancel(123).await.unwrap();
```


## Quick Links 
- [Back: Order Items](order_items.md)
- [Next: Subscription Invoices](subscription_invoice.md)