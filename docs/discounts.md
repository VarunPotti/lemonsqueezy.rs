# Discounts

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/usage-records#the-usage-record-object)

## Retrieve Discount

```rust
use lemonsqueezy::discounts::Discount;

let discounts = Discount::build(lemonsqueezy);
let discounts = discounts.retrieve(123).await.unwrap();
```

## Get All Discounts

```rust
use lemonsqueezy::discounts::Discount;

let discounts = Discount::build(lemonsqueezy);
let discounts = discounts.get_all(None).await.unwrap();
```

## Create a Discount
```rust
use lemonsqueezy::discount::{Discount, CreateDiscount, CreateDiscountAttributes, CreateDiscountRelationships};
let discount = Discount::build(lemonsqueezy);
let data = CreateDiscount {
   r#type: "discounts".to_string(),
   attributes: CreateDiscountAttributes {
      name: "Test".to_string(),
      code: "TEST".to_string(),
      amount: 100,
      amount_type: "fixed".to_string(),
      ..Default::default()
    },
    relationships: CreateDiscountRelationships {
     store: 1,
    ..Default::default()
    }
};
let discount = discount.create(data).await;
```

## Delete a Discount
```rust
use lemonsqueezy::discount::Discount;
let discount = Discount::build(lemonsqueezy);
let discount = discount.delete(1).await;
```

## Quick Links 
- [Back: Usage Records](usage_records.md)
- [Next: Discount Redemptions](discount_redemptions.md)