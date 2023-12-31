use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageRecordResponse {
    pub subscription_item_id: i64,
    pub quantity: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUsageRecord {
    pub data: CreateUsageRecordData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUsageRecordData {
    pub r#type: String,
    pub attributes: CreateUsageRecordAttributes,
    pub relationships: CreateUsageRecordRelationships,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUsageRecordAttributes {
    pub quantity: i64,
    pub action: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUsageRecordRelationships {
    #[serde(rename = "subscription-item")]
    pub subscription_item: CreateUsageRecordSubscriptionItem,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUsageRecordSubscriptionItem {
    pub data: CreateUsageRecordSubscriptionItemData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUsageRecordSubscriptionItemData {
    pub r#type: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageRecordPatchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

pub struct UsageRecordFilters {
    pub subscription_item_id: Option<i64>,
}

pub struct UsageRecords {
    pub(crate) api: crate::LemonSqueezy,
}

impl UsageRecords {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve a usage record
    ///
    /// # Arguments
    /// - usage_record_id: The ID of the usage record to retrieve
    ///
    /// # Returns
    /// - `anyhow::Result<Response<UsageRecordResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::usage_records::UsageRecords;
    /// let usage_records = UsageRecords::build(lemonsqueezy);
    /// let usage_record = usage_records.retrieve(1).await;
    /// ```
    ///
    pub async fn retrieve(
        &self,
        usage_record_id: usize,
    ) -> anyhow::Result<Response<UsageRecordResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<UsageRecordResponse>>(&format!("/v1/usage-records/{}", usage_record_id))
            .await?;

        Ok(response)
    }

    /// Retrieve all usage records
    ///
    /// # Arguments
    /// - filters: The usage record filters
    ///
    /// # Returns
    /// - `anyhow::Result<VecResponse<Vec<ResponseData<UsageRecordResponse>>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::usage_records::UsageRecords;
    /// let usage_records = UsageRecords::build(lemonsqueezy);
    /// let usage_records = usage_records.get_all(None).await;
    /// ```
    pub async fn get_all(
        &self,
        filters: Option<UsageRecordFilters>,
    ) -> anyhow::Result<
        VecResponse<Vec<ResponseData<UsageRecordResponse>>>,
        crate::errors::NetworkError,
    > {
        let mut url = "/v1/usage-records".to_string();

        if filters.is_some() {
            let filter: UsageRecordFilters = filters.unwrap();

            if let Some(subscription_item_id) = filter.subscription_item_id {
                url.push_str(&format!(
                    "?filter[subscription_item_id]={}",
                    subscription_item_id
                ));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }

    /// Update a usage record
    /// # Arguments
    /// - usage_record_id: The ID of the usage record to update
    /// - data: The usage record data
    ///
    /// # Returns
    /// - `anyhow::Result<Value, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::usage_records::UsageRecords;
    /// let usage_records = UsageRecords::build(lemonsqueezy);
    /// let usage_record = usage_records.create(usage_record_data).await;
    /// ```
    pub async fn create(
        &self,
        data: CreateUsageRecord,
    ) -> anyhow::Result<Value, crate::errors::NetworkError> {
        let response = self.api.post("/v1/usage-records", data).await?;

        Ok(response)
    }
}
