pub use crate::types::webhook::*;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{Response, ResponseData, VecResponse};

pub struct WebhookRedemptionsFilters {
    pub order_id: Option<i64>,
    pub discount_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponse {
    pub store_id: i64,
    pub url: String,
    pub events: Vec<String>,
    pub last_sent_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub test_mode: bool,
}

pub struct Webhook {
    pub(crate) api: crate::LemonSqueezy,
}

impl Webhook {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        webhook_id: i64,
    ) -> anyhow::Result<Response<WebhookResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get(&format!("/v1/webhooks/{}", webhook_id))
            .await?;

        Ok(response)
    }

    pub async fn delete(&self, webhook_id: i64) -> anyhow::Result<(), crate::errors::NetworkError> {
        self.api
            .delete(&format!("/v1/webhooks/{}", webhook_id))
            .await?;

        Ok(())
    }

    pub async fn get_all(
        &self,
        filters: Option<WebhookRedemptionsFilters>,
    ) -> anyhow::Result<VecResponse<Vec<ResponseData<WebhookResponse>>>, crate::errors::NetworkError>
    {
        let mut url = "/v1/webhooks".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: WebhookRedemptionsFilters = filters.unwrap();

            let mut added = false;

            if let Some(discount_id) = filter.discount_id {
                url.push_str(&format!("?filter[discount_id]={}", discount_id));

                added = true;
            }

            if let Some(order_id) = filter.order_id {
                if added {
                    url.push_str(&format!("&filter[order_id]={}", order_id));
                } else {
                    url.push_str(&format!("?filter[order_id]={}", order_id));
                }
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }

    pub async fn create(
        &self,
        data: CreateWebhook,
    ) -> anyhow::Result<Response<WebhookResponse>, crate::errors::NetworkError> {
        let data = json!({
            "data": data,
        });

        let response = self.api.post("/v1/webhooks", data).await?;

        Ok(response)
    }

    pub async fn update(
        &self,
        data: UpdateWebhook,
    ) -> anyhow::Result<Response<WebhookResponse>, crate::errors::NetworkError> {
        let url = format!("/v1/webhooks/{}", data.id);

        let data = json!({ "data": data });

        println!("{}", serde_json::to_string_pretty(&data).unwrap());

        let response = self.api.post(&url, data).await?;

        Ok(response)
    }
}
