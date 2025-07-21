# License Keys

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/license-keys#the-license-key-object)

## Activate License Keys

```rust
use lemonsqueezy::license_keys::LicenseKey;


let license_keys = LicenseKey::build(lemonsqueezy);
let activation_response = license_keys.activate("your-license-key", "InstanceName").await?;
```
## Deactivate License Keys

```rust
use lemonsqueezy::license_keys::LicenseKey;

let license_keys = LicenseKey::build(lemonsqueezy);
let deactivation_response = license_keys
    .deactivate("your-license-key", "your-instance-id")
    .await?;
```

## Validate License Keys

```rust
use lemonsqueezy::license_keys::LicenseKey;

let license_keys = LicenseKey::build(lemonsqueezy);
// With instance_id
let validation_response = license_keys
    .validate("your-license-key", Some("your-instance-id"))
    .await?;
// Without instance_id
let validation_response = license_keys
    .validate("your-license-key", None)
    .await?;
```

## Retrieve License Keys

```rust
use lemonsqueezy::license_keys::LicenseKey;

let license_key = LicenseKey::build(lemonsqueezy);
let license_key = license_key.retrieve(123).await.unwrap();
```

## Get All License Keys

```rust
use lemonsqueezy::license_keys::LicenseKey;

let license_keys = LicenseKey::build(lemonsqueezy);
let license_keys = license_keys.get_all(None).await.unwrap();
```

## Quick Links 
- [Back: Discount Redemptions](discount_redemptions.md)
- [Next: License Key Instances](license_key_instances.md)