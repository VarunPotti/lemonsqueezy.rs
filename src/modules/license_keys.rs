use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseKeyResponse {
    pub store_id: i64,
    pub customer_id: i64,
    pub order_id: i64,
    pub order_item_id: i64,
    pub product_id: i64,
    pub user_name: String,
    pub user_email: String,
    pub key: String,
    pub key_short: String,
    pub activation_limit: i64,
    pub instances_count: i64,
    pub disabled: bool,//i64,
    pub status: String,
    pub status_formatted: String,
    pub expires_at: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct LicenseKeyFilters {
    pub store_id: Option<i64>,
    pub order_id: Option<i64>,
    pub order_item_id: Option<i64>,
    pub product_id: Option<i64>,
}

pub struct LicenseKey {
    pub(crate) api: crate::LemonSqueezy,
}

impl LicenseKey {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve a license key
    ///
    /// # Arguments
    /// - license_key_id: The ID of the license key to retrieve
    ///
    /// # Returns
    /// - `anyhow::Result<Response<LicenseKeyResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::license_keys::LicenseKey;
    /// let license_keys = LicenseKey::build(lemonsqueezy);
    /// let license_key = license_keys.retrieve(1).await;
    /// ```
    pub async fn retrieve(
        &self,
        license_key_id: usize,
    ) -> anyhow::Result<Response<LicenseKeyResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<LicenseKeyResponse>>(&format!("/v1/license-keys/{}", license_key_id))
            .await?;

        Ok(response)
    }

    /// Retrieve all license keys
    ///
    /// # Arguments
    /// - filters: The license key filters
    ///
    /// # Returns
    /// - `anyhow::Result<VecResponse<Vec<ResponseData<LicenseKeyResponse>>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::license_keys::LicenseKey;
    /// let license_keys = LicenseKey::build(lemonsqueezy);
    /// let license_key = license_keys.get_all(None).await;
    /// ```
    pub async fn get_all(
        &self,
        filters: Option<LicenseKeyFilters>,
    ) -> anyhow::Result<
        VecResponse<Vec<ResponseData<LicenseKeyResponse>>>,
        crate::errors::NetworkError,
    > {
        let mut url = "/v1/license-keys".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: LicenseKeyFilters = filters.unwrap();

            let mut added = false;

            if let Some(store_id) = filter.store_id {
                url.push_str(&format!("?filter[store_id]={}", store_id));

                added = true;
            }

            if let Some(order_id) = filter.order_id {
                if added {
                    url.push_str(&format!("&filter[order_id]={}", order_id));
                } else {
                    url.push_str(&format!("?filter[order_id]={}", order_id));
                }

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
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
