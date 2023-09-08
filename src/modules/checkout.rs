pub use crate::types::checkout::*;

use serde_json::json;

use crate::utils::{Response, ResponseData, VecResponse};

pub struct WebhookRedemptionsFilters {
    pub store_id: Option<i64>,
}

pub struct Checkout {
    pub(crate) api: crate::LemonSqueezy,
}

impl Checkout {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        checkout_id: String,
    ) -> anyhow::Result<Response<CheckoutResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<CheckoutResponse>>(&format!("/v1/checkouts/{}", checkout_id))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<WebhookRedemptionsFilters>,
    ) -> anyhow::Result<VecResponse<Vec<ResponseData<CheckoutResponse>>>, crate::errors::NetworkError>
    {
        let mut url = "/v1/checkouts".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: WebhookRedemptionsFilters = filters.unwrap();

            if let Some(store_id) = filter.store_id {
                url.push_str(&format!("?filter[store_id]={}", store_id));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }

    pub async fn create(
        &self,
        data: CreateCheckout,
    ) -> anyhow::Result<Response<CheckoutResponse>, crate::errors::NetworkError> {
        let data = json!({
            "data": data,
        });

        println!("{}", serde_json::to_string_pretty(&data).unwrap());

        let response = self.api.post("/v1/checkouts", data).await?;

        Ok(response)
    }
}
