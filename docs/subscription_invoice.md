# Subscription Invoice

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/subscription-invoices#the-subscription-invoice-object)

## Retrieve Subscription Invoice

```rust
use lemonsqueezy::subscription_invoice::SubscriptionInvoice;
let subscriptions = SubscriptionInvoice::build(lemonsqueezy);
let subscriptions = subscriptions.retrieve(1).await;
```

## Get All Subscription Invoice

```rust
use lemonsqueezy::subscription_invoice::SubscriptionInvoice;

let mut filters = SubscriptionInvoiceFilter::default();
filters.store_id = Some(1);
let subscription_invoice = SubscriptionInvoice::build(lemonsqueezy);
let subscription_invoice = subscription_invoice.get_all(Some(filters)).await;
```

## Quick Links 
- [Back: Subscriptions](subscriptions.md)
- [Next: Subscription Items](subscription_items.md)