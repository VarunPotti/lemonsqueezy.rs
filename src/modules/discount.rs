use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    types::discount::CreateDiscountRelationships,
    utils::{Data, Response, VecResponse},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscountResponse {
    pub store_id: i64,
    pub name: String,
    pub code: String,
    pub amount: i64,
    pub amount_type: String,
    pub is_limited_to_products: bool,
    pub is_limited_redemptions: bool,
    pub max_redemptions: i64,
    pub starts_at: Value,
    pub expires_at: Value,
    pub duration: String,
    pub duration_in_months: i64,
    pub status: String,
    pub status_formatted: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDiscount {
    pub r#type: String,
    pub attributes: CreateDiscountAttributes,
    pub relationships: CreateDiscountRelationships,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDiscountAttributes {
    pub name: String,
    pub code: String,
    pub amount: i64,
    /// percent or fixed
    pub amount_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_limited_to_products: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_limited_redemptions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// once, repeating, forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

pub struct DiscountFilters {
    pub store_id: Option<i64>,
}

pub struct Discount {
    pub(crate) api: crate::LemonSqueezy,
}

impl Discount {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        usage_record_id: usize,
    ) -> anyhow::Result<Response<DiscountResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<DiscountResponse>>(&format!("/v1/discounts/{}", usage_record_id))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<DiscountFilters>,
    ) -> anyhow::Result<VecResponse<Vec<Data<DiscountResponse>>>, crate::errors::NetworkError> {
        let mut url = "/v1/discounts".to_string();

        if filters.is_some() {
            let filter: DiscountFilters = filters.unwrap();

            if let Some(store_id) = filter.store_id {
                url.push_str(&format!("?filter[store_id]={}", store_id));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }

    pub async fn delete(
        &self,
        usage_record_id: usize,
    ) -> anyhow::Result<Response<DiscountResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .delete::<Response<DiscountResponse>>(&format!("/v1/discounts/{}", usage_record_id))
            .await?;

        Ok(response)
    }

    pub async fn create(
        &self,
        data: CreateDiscount,
    ) -> anyhow::Result<Response<DiscountResponse>, crate::errors::NetworkError> {
        let json = json!({ "data": data });
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        let response = self.api.post("/v1/discounts", json).await?;

        Ok(response)
    }
}
