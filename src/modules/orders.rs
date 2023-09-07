use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{Data, Response, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderResponse {
    pub store_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub identifier: String,
    pub order_number: Option<i64>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub currency: String,
    pub currency_rate: String,
    pub subtotal: Option<i64>,
    pub discount_total: Option<i64>,
    pub tax: Option<i64>,
    pub total: Option<i64>,
    pub subtotal_usd: Option<i64>,
    pub discount_total_usd: Option<i64>,
    pub tax_usd: Option<i64>,
    pub total_usd: Option<i64>,
    pub tax_name: Option<String>,
    pub tax_rate: Option<String>,
    pub status: Option<String>,
    pub status_formatted: Option<String>,
    pub refunded: bool,
    pub refunded_at: Option<Value>,
    pub subtotal_formatted: Option<String>,
    pub discount_total_formatted: Option<String>,
    pub tax_formatted: Option<String>,
    pub total_formatted: Option<String>,
    pub first_order_item: Option<FirstOrderItem>,
    pub urls: Urls,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstOrderItem {
    pub id: Option<i64>,
    pub order_id: Option<i64>,
    pub product_id: Option<i64>,
    pub variant_id: Option<i64>,
    pub product_name: String,
    pub variant_name: String,
    pub price: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
    pub test_mode: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    pub receipt: String,
}

pub struct OrderFilters {
    pub store_id: Option<i64>,
    pub user_email: Option<String>,
}

pub struct Orders {
    pub(crate) api: crate::LemonSqueezy,
}

impl Orders {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        file_id: usize,
    ) -> anyhow::Result<Response<OrderResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<OrderResponse>>(&format!("/v1/orders/{}", file_id))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<OrderFilters>,
    ) -> anyhow::Result<VecResponse<Vec<Data<OrderResponse>>>, crate::errors::NetworkError> {
        let mut url = "/v1/orders".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: OrderFilters = filters.unwrap();

            let mut added = false;

            if let Some(store_id) = filter.store_id {
                url.push_str(&format!("?filter[store_id]={}", store_id));

                added = true;
            }

            if let Some(user_email) = filter.user_email {
                if added {
                    url.push_str(&format!("&filter[user_email]={}", user_email));
                } else {
                    url.push_str(&format!("?filter[user_email]={}", user_email));
                }
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
