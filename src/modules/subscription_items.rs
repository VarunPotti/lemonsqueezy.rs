use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::{Response, ResponseData, ResponseMeta, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionItemResponse {
    pub subscription_id: i64,
    pub usage_id: Option<i64>,
    pub price_id: i64,
    pub quantity: i64,
    pub is_usage_based: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionItemMetaResponse {
    pub period_start: String,
    pub period_end: String,
    pub quantity: i64,
    pub interval_unit: String,
    pub interval_quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionItemPatchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

pub struct SubscriptionItemFilters {
    pub store_id: Option<i64>,
    pub order_id: Option<i64>,
    pub order_item_id: Option<i64>,
    pub product_id: Option<i64>,
}

pub struct SubscriptionItems {
    pub(crate) api: crate::LemonSqueezy,
}

impl SubscriptionItems {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve a subscription item
    ///
    /// # Arguments
    /// - subscription_item_id: The ID of the subscription item to retrieve
    ///
    /// # Returns
    /// - `anyhow::Result<Response<SubscriptionItemResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::subscription_items::SubscriptionItems;
    /// let subscription_items = SubscriptionItems::build(lemonsqueezy);
    /// let subscriptions_item = subscription_items.retrieve(1).await;
    /// ```
    pub async fn retrieve(
        &self,
        subscription_item_id: usize,
    ) -> anyhow::Result<Response<SubscriptionItemResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<SubscriptionItemResponse>>(&format!(
                "/v1/subscription-items/{}",
                subscription_item_id
            ))
            .await?;

        Ok(response)
    }

    /// Retrieve all subscription items
    ///
    /// # Arguments
    /// - filters: The subscription item filters
    ///
    /// # Returns
    /// - `anyhow::Result<VecResponse<Vec<ResponseData<SubscriptionItemResponse>>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::subscription_items::SubscriptionItems;
    /// let subscription_items = SubscriptionItems::build(lemonsqueezy);
    /// let subscription_items = subscription_items.get_all(None).await.unwrap();
    /// ```
    pub async fn get_all(
        &self,
        filters: Option<SubscriptionItemFilters>,
    ) -> anyhow::Result<
        VecResponse<Vec<ResponseData<SubscriptionItemResponse>>>,
        crate::errors::NetworkError,
    > {
        let mut url = "/v1/subscription-items".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: SubscriptionItemFilters = filters.unwrap();

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

    /// Update a subscription item
    ///
    /// # Arguments
    /// - data: The subscription item data
    ///
    /// # Returns
    /// - `anyhow::Result<Value, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::subscription_items::SubscriptionItems;
    /// let subscription_items = SubscriptionItems::build(lemonsqueezy);
    /// let subscription_items = subscription_items.update(data).await.unwrap();
    /// ```
    pub async fn update(
        &self,
        data: ResponseData<SubscriptionItemPatchRequest>,
    ) -> anyhow::Result<Value, crate::errors::NetworkError> {
        let reqwest_body =
            reqwest::Body::from(serde_json::to_string(&json!({ "data": data })).unwrap());

        let response = self
            .api
            .patch(&format!("/v1/subscription-items/{}", data.id), reqwest_body)
            .await?;

        Ok(response)
    }

    /// Retrieve the current usage for a subscription item
    ///
    /// # Arguments
    /// - subscription_item_id: The ID of the subscription item to retrieve
    ///
    /// # Returns
    /// - anyhow::Result<ResponseMeta<SubscriptionItemMetaResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::subscription_items::SubscriptionItems;
    /// let subscription_items = SubscriptionItems::build(lemonsqueezy);
    /// let subscription_items = subscription_items.current_usage(1).await.unwrap();
    /// ```
    pub async fn current_usage(
        &self,
        subscription_item_id: usize,
    ) -> anyhow::Result<ResponseMeta<SubscriptionItemMetaResponse>, crate::errors::NetworkError>
    {
        let response = self
            .api
            .get(&format!(
                "/v1/subscription-items/{}/current-usage",
                subscription_item_id
            ))
            .await?;

        Ok(response)
    }
}
