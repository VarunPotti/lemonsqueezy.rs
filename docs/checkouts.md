# Checkouts

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using.
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/checkouts#the-checkout-object)

## Retrieve Checkouts

```rust
use lemonsqueezy::checkout::Checkout;

let checkout = Checkout::build(lemonsqueezy);
let checkout = checkout.retrieve(123).await.unwrap();
```

## Get All Checkouts

```rust
use lemonsqueezy::checkout::Checkout;

let checkout = Checkout::build(lemonsqueezy);
let checkout = checkout.get_all(None).await.unwrap();
```

## Create a Checkout

```rust
use lemonsqueezy::checkout::Checkout;
use lemonsqueezy::types::checkout::*;
let checkout = Checkout::build(lemonsqueezy);

let checkout = checkout
    .create(CreateCheckout {
        r#type: String::from("checkouts"),
        attributes: CreateCheckoutAttributes {
            ..Default::default()
        },
        relationships: Some(CreateCheckoutRelationships {
            store: lemonsqueezy::types::Data {
                data: CreateCheckoutRelationShipData {
                    r#type: String::from("stores"),
                    id: String::from("//store id"),
                },
            },
            variant: lemonsqueezy::types::Data {
                data: CreateCheckoutRelationShipData {
                    r#type: String::from("variants"),
                    id: String::from("// variant id"),
                },
            },
        }),
    }).await;
```

## Quick Links

- [Back: License Key Instances](license_key_instances.md)
- [Next: Webhooks](webhooks.md)
