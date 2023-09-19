# Files

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/files)

## Retrieve File

```rust
use lemonsqueezy::files::File;

let file = File::build(lemonsqueezy);
let file = file.retrieve(123).await.unwrap();
```

## Get All Files

```rust
use lemonsqueezy::files::File;

let files = File::build(lemonsqueezy);
let files = files.get_all(None).await.unwrap();
```

## Quick Links 
- [Back: Variants](variants.md)
- [Next: Orders](orders.md)