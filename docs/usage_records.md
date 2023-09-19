# Usage Records

The stores endpoint allows you to retrieve information about the stores associated with the API key you are using. 
You can find the LemonSqueezy link [here](https://docs.lemonsqueezy.com/api/usage-records#the-usage-record-object)

## Retrieve Usage Records

```rust
use lemonsqueezy::usage_records::UsageRecords;

let usage_records = UsageRecords::build(lemonsqueezy);
let subscription = usage_records.retrieve(123).await.unwrap();
```

## Get All Usage Records

```rust
use lemonsqueezy::usage_records::UsageRecords;

let usage_records = UsageRecords::build(lemonsqueezy);
let usage_records = usage_records.get_all(None).await.unwrap();
```

## Create a Usage Record
```rust
use lemonsqueezy::usage_records::{UsageRecords, CreateUsageRecord};
use lemonsqueezy::utils::ResponseData;

let usage_records = UsageRecords::build(lemonsqueezy);
let subscription_data = CreateUsageRecord {
    ...
}
let usage_records = usage_records.update(subscription_data).await.unwrap();
```


## Quick Links 
- [Back: Subscription Items](subscription_items.md)
- [Next: Discounts](discounts.md)