# lemonsqueezy.rs

### Example Usage
~~~rust
use lemonsqueezy::modules::products::{Product, ProductFilters};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let lemonsqueezy = lemonsqueezy::LemonSqueezy::new(std::env::var("API_KEY").unwrap());

    let product = Product::build(lemonsqueezy);

    let product_filters = ProductFilters {
        store_id: Some(42756),
    };

    let products = product.get_all(Some(product_filters)).await.unwrap();

    println!("{:#?}", products);
}
~~~

### Docs
Docs can be found [here](docs/index.md)

### License
This project is licensed under the Apache License - see the [LICENSE.md](LICENSE.md) file for details