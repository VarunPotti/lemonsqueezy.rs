use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::constants::{Data, Response, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tier {
    pub last_unit: Option<Value>,
    pub unit_price: Option<i64>,
    pub fixed_fee: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceResponse {
    pub variant_id: i64,
    pub category: String,
    pub scheme: String,
    pub usage_aggregation: Value,
    pub unit_price: i64,
    pub package_size: Option<i64>,
    pub tiers: Option<Vec<Tier>>,
    pub renewal_interval_unit: Option<String>,
    pub renewal_interval_quantity: Option<i64>,
    pub trial_interval_unit: Option<String>,
    pub trial_interval_quantity: Option<i64>,
    pub min_price: Option<Value>,
    pub suggested_price: Option<Value>,
    pub tax_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct PriceFilters {
    pub variant_id: Option<i64>,
}

pub struct Prices {
    pub(crate) api: crate::LemonSqueezy,
}

impl Prices {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        price_id: usize,
    ) -> anyhow::Result<Response<PriceResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<PriceResponse>>(&format!("/v1/prices/{}", price_id))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<PriceFilters>,
    ) -> anyhow::Result<VecResponse<Vec<Data<PriceResponse>>>, crate::errors::NetworkError> {
        let mut url = "/v1/prices".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: PriceFilters = filters.unwrap();

            if let Some(store_id) = filter.variant_id {
                url.push_str(&format!("?filter[variant_id]={}", store_id));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
