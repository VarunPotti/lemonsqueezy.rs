use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Data;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct CheckoutResponse {
    pub store_id: i64,
    pub variant_id: i64,
    pub custom_price: Value,
    pub product_options: ProductOptions,
    pub checkout_options: CheckoutOptions,
    pub checkout_data: CheckoutData,
    // can either be false or Preview
    pub preview: Value,
    pub expires_at: Value,
    pub created_at: String,
    pub updated_at: String,
    pub test_mode: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct ProductOptions {
    pub name: String,
    pub description: String,
    pub media: Vec<Value>,
    pub redirect_url: String,
    pub receipt_button_text: String,
    pub receipt_link_url: String,
    pub receipt_thank_you_note: String,
    pub enabled_variants: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct CheckoutOptions {
    pub embed: bool,
    pub media: bool,
    pub logo: bool,
    pub desc: bool,
    pub discount: bool,
    pub dark: bool,
    pub subscription_preview: bool,
    pub button_color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct CheckoutData {
    pub email: String,
    pub name: String,
    pub billing_address: Vec<BillingAddress>,
    pub tax_number: String,
    pub discount_code: String,
    pub custom: Vec<Value>,
    pub variant_quantities: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct BillingAddress {
    pub country: String,
    pub zip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Preview {
    pub currency: String,
    pub currency_rate: i64,
    pub subtotal: i64,
    pub discount_total: i64,
    pub tax: i64,
    pub total: i64,
    pub subtotal_usd: i64,
    pub discount_total_usd: i64,
    pub tax_usd: i64,
    pub total_usd: i64,
    pub subtotal_formatted: String,
    pub discount_total_formatted: String,
    pub tax_formatted: String,
    pub total_formatted: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCheckoutRelationShip {
    r#type: String,
    id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCheckout {
    pub r#type: String,
    pub attributes: CreateCheckoutAttributes,
    pub relationships: Option<CreateCheckoutRelationships>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCheckoutAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_price: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_options: Option<CreateCheckoutProductOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_options: Option<CreateCheckoutCheckoutOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_data: Option<CreateCheckoutCheckoutData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct CreateCheckoutProductOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_button_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_link_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_thank_you_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_variants: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCheckoutCheckoutOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_color: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCheckoutCheckoutData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_quantities: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCheckoutBillingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCheckoutRelationships {
    pub store: Data<CreateCheckoutRelationShipData>,
    pub variant: Data<CreateCheckoutRelationShipData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCheckoutRelationShipData {
    pub r#type: String,
    pub id: String,
}
