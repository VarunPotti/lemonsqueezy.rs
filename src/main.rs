use lemonsqueezy::modules::order_items::OrderItem;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let lemonsqueezy = lemonsqueezy::LemonSqueezy::new(std::env::var("API_KEY").unwrap());

    let variant = OrderItem::build(lemonsqueezy);
    let variant = variant.get_all(None).await.unwrap();

    println!("{:#?}", variant);
}
