use serde::{Deserialize, Serialize};

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderItemResponse {
    #[serde(rename = "order_id")]
    pub order_id: i64,
    #[serde(rename = "product_id")]
    pub product_id: i64,
    #[serde(rename = "variant_id")]
    pub variant_id: i64,
    #[serde(rename = "product_name")]
    pub product_name: String,
    #[serde(rename = "variant_name")]
    pub variant_name: String,
    pub price: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub struct OrderItemFilters {
    pub order_id: Option<i64>,
    pub product_id: Option<i64>,
}

pub struct OrderItem {
    pub(crate) api: crate::LemonSqueezy,
}

impl OrderItem {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        file_id: usize,
    ) -> anyhow::Result<Response<OrderItemResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<OrderItemResponse>>(&format!("/v1/order-items/{}", file_id))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<OrderItemFilters>,
    ) -> anyhow::Result<
        VecResponse<Vec<ResponseData<OrderItemResponse>>>,
        crate::errors::NetworkError,
    > {
        let mut url = "/v1/order-items".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: OrderItemFilters = filters.unwrap();

            let mut added = false;

            if let Some(order_id) = filter.order_id {
                url.push_str(&format!("?filter[order_id]={}", order_id));

                added = true;
            }

            if let Some(product_id) = filter.product_id {
                if added {
                    url.push_str(&format!("&filter[product_id]={}", product_id));
                } else {
                    url.push_str(&format!("?filter[product_id]={}", product_id));
                }
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
