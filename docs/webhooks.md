# Webhooks

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/webhooks#create-a-webhook)

## Retrieve Webhooks

```rust
use lemonsqueezy::webhook::Webhook;

let webhook = Webhook::build(lemonsqueezy);
let webhook = webhook.retrieve(123).await.unwrap();
```

## Get All Webhooks

```rust
use lemonsqueezy::webhook::Webhook;

let webhook = Webhook::build(lemonsqueezy);
let webhook = webhook.get_all(None).await.unwrap();
```

## Delete a Webhook
```rust
use lemonsqueezy::webhook::Webhook;

let webhook = Webhook::build(lemonsqueezy);
let webhook = webhook.delete(123).await.unwrap();
```

## Update a Webhook
```rust
use lemonsqueezy::webhook::Webhook;
let webhook = Webhook::build(lemonsqueezy);
let webhook = webhook.update(data).await;
```

## Create Webhook
```rust
use lemonsqueezy::webhook::Webhook;
use lemonsqueezy::types::webhook::*;

let webhook = Webhook::build(lemonsqueezy);
let webhook = webhook.create(CreateWebhook {
   store_id: 1,
    // ..
}).await;
```


## Quick Links 
- [Back: Checkouts](checkouts.md)