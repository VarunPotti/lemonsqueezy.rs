use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{Data, Response, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductResponse {
    pub store_id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub status_formatted: Option<String>,
    pub thumb_url: Option<String>,
    pub large_thumb_url: Option<String>,
    pub price: Option<i64>,
    pub pay_what_you_want: Option<bool>,
    pub from_price: Option<Value>,
    pub to_price: Option<Value>,
    pub buy_now_url: Option<String>,
    pub price_formatted: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct ProductFilters {
    pub store_id: Option<i64>,
}

pub struct Product {
    pub(crate) api: crate::LemonSqueezy,
}

impl Product {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        product_id: usize,
    ) -> anyhow::Result<Response<ProductResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<ProductResponse>>(&format!("/v1/products/{}", product_id))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<ProductFilters>,
    ) -> anyhow::Result<VecResponse<Vec<Data<ProductResponse>>>, crate::errors::NetworkError> {
        let mut url = "/v1/products".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: ProductFilters = filters.unwrap();

            if let Some(store_id) = filter.store_id {
                url.push_str(&format!("?filter[store_id]={}", store_id));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
