use lemonsqueezy::modules::usage_records::UsageRecords;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let lemonsqueezy = lemonsqueezy::LemonSqueezy::new(std::env::var("API_KEY").unwrap());

    let product = UsageRecords::build(lemonsqueezy.clone());

    let products = product.get_all(None).await.unwrap();

    println!("{:#?}", products);
}
