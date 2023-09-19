# Discount Redemptions

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/discount-redemptions#the-discount-redemption-object)

## Retrieve Discount Redemptions

```rust
use lemonsqueezy::discount_redemptions::DiscountRedemptions;

let file = DiscountRedemptions::build(lemonsqueezy);
let file = file.retrieve(123).await.unwrap();
```

## Get All Discount Redemptions

```rust
use lemonsqueezy::files::DiscountRedemptions;

let files = DiscountRedemptions::build(lemonsqueezy);
let files = files.get_all(None).await.unwrap();
```

## Quick Links 
- [Back: Discounts](discounts.md)
- [Next: Customers](license_keys.md)