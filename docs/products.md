# Products

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/products)

## Retrieve Product

```rust
use lemonsqueezy::product::Product;

let product = Product::build(lemonsqueezy);
let product = product.retrieve(123).await.unwrap();
```

## Get All Products

```rust
use lemonsqueezy::product::Product;

let product = Product::build(lemonsqueezy);
let product = product.get_all().await.unwrap();
```

## Quick Links 
- [Back: Installation](index.md)
- [Next: Variants](variants.md)