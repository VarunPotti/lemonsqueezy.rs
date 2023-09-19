# Variants

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/variants)

## Retrieve Variant

```rust
use lemonsqueezy::variant::Variant;

let variant = Variant::build(lemonsqueezy);
let variant = variant.retrieve(123).await.unwrap();
```

## Get All Variants

```rust
use lemonsqueezy::variant::Variant;

let variant = Variant::build(lemonsqueezy);
let variant = variant.get_all(None).await.unwrap();
```

## Quick Links 
- [Back: Products](products.md)
- [Next: Files](files.md)