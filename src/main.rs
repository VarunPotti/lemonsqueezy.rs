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
