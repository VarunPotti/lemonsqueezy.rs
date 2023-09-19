use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    pub store_id: i64,
    pub customer_id: i64,
    pub order_id: i64,
    pub order_item_id: i64,
    pub product_id: i64,
    pub variant_id: i64,
    pub product_name: String,
    pub variant_name: String,
    pub user_name: String,
    pub user_email: String,
    pub status: String,
    pub status_formatted: String,
    pub card_brand: String,
    pub card_last_four: String,
    pub pause: Option<Value>,
    pub cancelled: bool,
    pub trial_ends_at: Option<Value>,
    pub billing_anchor: i64,
    pub first_subscription_item: FirstSubscriptionItem,
    pub urls: Urls,
    pub renews_at: String,
    pub ends_at: Value,
    pub created_at: String,
    pub updated_at: String,
    pub test_mode: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionPatchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_anchor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_immediately: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_prorations: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstSubscriptionItem {
    pub id: i64,
    pub subscription_id: i64,
    pub price_id: i64,
    pub quantity: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    pub update_payment_method: String,
}

pub struct SubscriptionFilters {
    pub store_id: Option<i64>,
    pub order_id: Option<i64>,
    pub order_item_id: Option<i64>,
    pub product_id: Option<i64>,
}

pub struct Subscriptions {
    pub(crate) api: crate::LemonSqueezy,
}

impl Subscriptions {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve a Subscription
    ///
    /// # Arguments
    /// * `subscription_id` - The subscription id
    ///
    /// # Returns
    /// `Result<Response<SubscriptionResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```rust
    /// use lemonsqueezy::subscriptions::Subscriptions;
    /// let subscriptions = Subscriptions::build(lemonsqueezy);
    /// let subscriptions = subscriptions.retrieve(123).await.unwrap();
    /// ```
    ///
    /// Read More: https://docs.lemonsqueezy.com/api/subscriptions#retrieve-a-subscription
    pub async fn retrieve(
        &self,
        subscription_id: usize,
    ) -> anyhow::Result<Response<SubscriptionResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<SubscriptionResponse>>(&format!(
                "/v1/subscriptions/{}",
                subscription_id
            ))
            .await?;

        Ok(response)
    }

    /// Get all the Subscriptions
    ///
    /// # Arguments
    /// * `filters` - The Subscription filters
    ///
    /// # Returns
    /// anyhow::Result<VecResponse<Vec<ResponseData<SubscriptionResponse>>>,crate::errors::NetworkError>
    ///
    /// # Example
    /// ```rust
    /// use lemonsqueezy::subscriptions::Subscriptions;
    /// let subscriptions = Subscriptions::build(lemonsqueezy);
    /// let subscriptions = subscriptions.get_all(None).await.unwrap();
    /// ```
    ///
    /// Read More: https://docs.lemonsqueezy.com/api/subscriptions#list-all-subscriptions
    pub async fn get_all(
        &self,
        filters: Option<SubscriptionFilters>,
    ) -> anyhow::Result<
        VecResponse<Vec<ResponseData<SubscriptionResponse>>>,
        crate::errors::NetworkError,
    > {
        let mut url = "/v1/subscriptions".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: SubscriptionFilters = filters.unwrap();

            let mut added = false;

            if let Some(order_id) = filter.order_id {
                url.push_str(&format!("?filter[order_id]={}", order_id));

                added = true;
            }

            if let Some(order_item_id) = filter.order_item_id {
                if added {
                    url.push_str(&format!("&filter[order_item_id]={}", order_item_id));
                } else {
                    url.push_str(&format!("?filter[order_item_id]={}", order_item_id));
                }

                added = true;
            }

            if let Some(product_id) = filter.product_id {
                if added {
                    url.push_str(&format!("&filter[product_id]={}", product_id));
                } else {
                    url.push_str(&format!("?filter[product_id]={}", product_id));
                }

                added = true;
            }

            if let Some(store_id) = filter.store_id {
                if added {
                    url.push_str(&format!("&filter[store_id]={}", store_id));
                } else {
                    url.push_str(&format!("?filter[store_id]={}", store_id));
                }
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }

    /// Update a Subscription
    ///
    /// # Arguments
    /// * `data` - ResponseData<SubscriptionPatchRequest>,
    ///
    /// # Returns
    /// anyhow::Result<Value, crate::errors::NetworkError>
    ///
    /// # Example
    /// ```rust
    /// use lemonsqueezy::subscriptions::Subscriptions;
    /// let subscriptions = Subscriptions::build(lemonsqueezy);
    /// let subscriptions = subscriptions.update(data).await.unwrap();
    /// ```
    ///
    /// Read More: https://docs.lemonsqueezy.com/api/subscriptions#update-a-subscription
    pub async fn update(
        &self,
        data: ResponseData<SubscriptionPatchRequest>,
    ) -> anyhow::Result<Value, crate::errors::NetworkError> {
        let reqwest_body =
            reqwest::Body::from(serde_json::to_string(&json!({ "data": data })).unwrap());

        let response = self
            .api
            .patch(&format!("/v1/subscriptions/{}", data.id), reqwest_body)
            .await?;

        Ok(response)
    }

    /// Cancel a Subscription
    ///
    /// # Arguments
    /// * `subscription_id` - The subscription id
    ///
    /// # Returns
    /// anyhow::Result<Value, crate::errors::NetworkError>
    ///
    /// # Example
    /// ```rust
    /// use lemonsqueezy::subscriptions::Subscriptions;
    /// let subscriptions = Subscriptions::build(lemonsqueezy);
    /// let subscriptions = subscriptions.cancel(123).await.unwrap();
    /// ```
    ///
    /// Read More: https://docs.lemonsqueezy.com/api/subscriptions#update-a-subscription
    pub async fn cancel(
        &self,
        subscription_id: usize,
    ) -> anyhow::Result<serde_json::Value, crate::errors::NetworkError> {
        let response = self
            .api
            .delete(&format!("/v1/subscriptions/{}", subscription_id))
            .await?;

        Ok(response)
    }
}
