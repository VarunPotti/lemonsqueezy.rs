use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::constants::{Data, Response, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionInvoiceResponse {
    pub store_id: i64,
    pub subscription_id: i64,
    pub customer_id: i64,
    pub user_name: String,
    pub user_email: String,
    pub billing_reason: String,
    pub card_brand: String,
    pub card_last_four: String,
    pub currency: String,
    pub currency_rate: String,
    pub subtotal: i64,
    pub discount_total: i64,
    pub tax: i64,
    pub total: i64,
    pub subtotal_usd: i64,
    pub discount_total_usd: i64,
    pub tax_usd: i64,
    pub total_usd: i64,
    pub status: String,
    pub status_formatted: String,
    pub refunded: bool,
    pub refunded_at: Option<Value>,
    pub subtotal_formatted: String,
    pub discount_total_formatted: String,
    pub tax_formatted: String,
    pub total_formatted: String,
    pub urls: Urls,
    pub created_at: String,
    pub updated_at: String,
    pub test_mode: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    pub invoice_url: String,
}

pub struct SubscriptionInvoiceFilter {
    pub store_id: Option<i64>,
    pub status: Option<String>,

    pub refunded: Option<bool>,
    pub subscription_id: Option<i64>,
}

pub struct SubscriptionInvoice {
    pub(crate) api: crate::LemonSqueezy,
}

impl SubscriptionInvoice {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        file_id: usize,
    ) -> anyhow::Result<Response<SubscriptionInvoiceResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<SubscriptionInvoiceResponse>>(&format!(
                "/v1/subscription-invoices/{}",
                file_id
            ))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<SubscriptionInvoiceFilter>,
    ) -> anyhow::Result<
        VecResponse<Vec<Data<SubscriptionInvoiceResponse>>>,
        crate::errors::NetworkError,
    > {
        let mut url = "/v1/subscription-invoices".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: SubscriptionInvoiceFilter = filters.unwrap();

            let mut added = false;

            if filter.store_id.is_some() {
                url = format!("{}?filter[store_id]={}", url, filter.store_id.unwrap());
                added = true;
            }

            if filter.status.is_some() {
                if added {
                    url = format!("{}&filter[status]={}", url, filter.status.unwrap());
                } else {
                    url = format!("{}?filter[status]={}", url, filter.status.unwrap());
                }
                added = true;
            }

            if filter.refunded.is_some() {
                if added {
                    url = format!("{}&filter[refunded]={}", url, filter.refunded.unwrap());
                } else {
                    url = format!("{}?filter[refunded]={}", url, filter.refunded.unwrap());
                }
                added = true;
            }

            if filter.subscription_id.is_some() {
                if added {
                    url = format!(
                        "{}&filter[subscription_id]={}",
                        url,
                        filter.subscription_id.unwrap()
                    );
                } else {
                    url = format!(
                        "{}?filter[subscription_id]={}",
                        url,
                        filter.subscription_id.unwrap()
                    );
                }
                added = true;
            }
        }

        println!("{}", url);

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
