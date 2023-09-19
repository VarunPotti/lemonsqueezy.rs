# License Key Instances

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/license-key-instances#the-license-key-instance-object)

## Retrieve License Key Instances

```rust
use lemonsqueezy::license_key_instances::LicenseKeyInstances;

let license_key_inst = LicenseKeyInstances::build(lemonsqueezy);
let license_key_inst = license_key_inst.retrieve(123).await.unwrap();
```

## Get All License Key Instances

```rust
use lemonsqueezy::license_key_inst_instances::LicenseKeyInstances;

let license_key_insts = LicenseKeyInstances::build(lemonsqueezy);
let license_key_insts = license_key_insts.get_all(None).await.unwrap();
```

## Quick Links 
- [Back: License Keys](license_keys.md)
- [Next: Checkouts](checkouts.md)