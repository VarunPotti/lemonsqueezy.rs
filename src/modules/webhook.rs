pub use crate::types::webhook::*;

pub use crate::orders::OrderResponse;
pub use crate::subscriptions::SubscriptionResponse;
pub use crate::license_keys::LicenseKeyResponse;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookOrderEvent<T> {
    pub meta: WebhookEventMeta<T>,
    pub data: WebhookOrderData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookSuscriptionEvent<T> {
    pub meta: WebhookEventMeta<T>,
    pub data: WebhookSubscriptionData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookLicenseEvent<T> {
    pub meta: WebhookEventMeta<T>,
    pub data: WebhookLicenseData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookEventMeta<T> {
    pub event_name: String,
    pub custom_data: T,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookOrderData {
    pub r#type: String,
    pub id: i64,
    pub attributes: OrderResponse,
    pub relationships: WebhookRelationships,
    pub links: WebhookLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookSubscriptionData {
    pub r#type: String,
    pub id: i64,
    pub attributes: SubscriptionResponse,
    pub relationships: WebhookRelationships,
    pub links: WebhookLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookLicenseData {
    pub r#type: String,
    pub id: i64,
    pub attributes: LicenseKeyResponse,
    pub relationships: WebhookRelationships,
    pub links: WebhookLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookRelationships {
    pub store: RelationshipLinks,
    pub customer: RelationshipLinks,
    pub order_items: RelationshipLinks,
    pub subscriptions: RelationshipLinks,
    pub license_keys: RelationshipLinks,
    #[serde(rename = "discount-redemptions")]
    pub discount_redemptions: RelationshipLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipLinks {
    pub related: String,
    #[serde(rename = "self")]
    pub link_self: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookLinks {
    #[serde(rename = "self")]
    pub link_self: String,
}

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

    /// Retrieve a webhook
    ///
    /// # Arguments
    /// - webhook_id: The ID of the webhook to retrieve
    ///
    /// # Returns
    /// - `anyhow::Result<Response<WebhookResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::webhook::Webhook;
    ///
    /// let webhook = Webhook::build(lemonsqueezy);
    /// let webhook = webhook.retrieve(1).await;
    /// ```
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

    /// Delete a webhook
    ///
    /// # Arguments
    /// - webhook_id: The ID of the webhook to delete
    ///
    /// # Returns
    /// - `anyhow::Result<(), crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::webhook::Webhook;
    ///
    /// let webhook = Webhook::build(lemonsqueezy);
    /// let webhook = webhook.delete(1).await;
    /// ```
    pub async fn delete(&self, webhook_id: i64) -> anyhow::Result<(), crate::errors::NetworkError> {
        self.api
            .delete(&format!("/v1/webhooks/{}", webhook_id))
            .await?;

        Ok(())
    }

    /// Retrieve all webhooks
    ///
    /// # Arguments
    /// - filters: The webhook filters
    ///
    /// # Returns
    /// - `anyhow::Result<VecResponse<Vec<ResponseData<WebhookResponse>>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::webhook::Webhook;
    ///
    /// let webhook = Webhook::build(lemonsqueezy);
    /// let webhook = webhook.get_all(None).await;
    /// ```
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

    /// Create a webhook
    ///
    /// # Arguments
    /// - data: The webhook data
    ///
    /// # Returns
    /// - `anyhow::Result<Response<WebhookResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::webhook::Webhook;
    /// use lemonsqueezy::types::webhook::*;
    ///
    /// let webhook = Webhook::build(lemonsqueezy);
    /// let webhook = webhook.create(CreateWebhook {
    ///    store_id: 1,
    ///     // ..
    /// }).await;
    /// ```
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

    /// Update a webhook
    ///
    /// # Arguments
    /// - data: The webhook data
    ///
    /// # Returns
    /// - `anyhow::Result<Response<WebhookResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::webhook::Webhook;
    ///
    /// let webhook = Webhook::build(lemonsqueezy);
    /// let webhook = webhook.update(data).await;
    /// ```
    pub async fn update(
        &self,
        data: UpdateWebhook,
    ) -> anyhow::Result<Response<WebhookResponse>, crate::errors::NetworkError> {
        let url = format!("/v1/webhooks/{}", data.id);

        let data = json!({ "data": data });

        let response = self.api.post(&url, data).await?;

        Ok(response)
    }
}
