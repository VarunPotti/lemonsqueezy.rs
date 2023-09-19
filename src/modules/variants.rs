use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantResponse {
    pub product_id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price: Option<i64>,
    pub is_subscription: Option<bool>,
    pub interval: Option<Value>,
    pub interval_count: Option<Value>,
    pub has_free_trial: Option<bool>,
    pub trial_interval: Option<String>,
    pub trial_interval_count: Option<i64>,
    pub pay_what_you_want: Option<bool>,
    pub min_price: Option<i64>,
    pub suggested_price: Option<i64>,
    pub has_license_keys: Option<bool>,
    pub license_activation_limit: Option<i64>,
    pub is_license_limit_unlimited: Option<bool>,
    pub license_length_value: Option<i64>,
    pub license_length_unit: Option<String>,
    pub is_license_length_unlimited: Option<bool>,
    pub sort: Option<i64>,
    pub status: Option<String>,
    pub status_formatted: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct VariantFilters {
    pub product_id: Option<i64>,
}

pub struct Variant {
    pub(crate) api: crate::LemonSqueezy,
}

impl Variant {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve the variant
    ///
    /// # Arguments
    /// * `variant_id` - The variant id
    ///
    /// # Returns
    /// `Result<Response<VariantResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```rust
    /// use lemonsqueezy::variant::Variant;
    /// let variants = Variant::build(lemonsqueezy);
    /// let variant = variants.retrieve(123).await.unwrap();
    /// ```
    ///
    /// Read More: https://docs.lemonsqueezy.com/api/variants
    pub async fn retrieve(
        &self,
        variant_id: usize,
    ) -> anyhow::Result<Response<VariantResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<VariantResponse>>(&format!("/v1/variants/{}", variant_id))
            .await?;

        Ok(response)
    }

    /// Get all variants
    ///
    /// # Returns
    /// `Result<VecResponse<Vec<ResponseData<CustomerResponse>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```rust
    /// use lemonsqueezy::variant::Variant;
    /// let variants = Variant::build(lemonsqueezy);
    /// let variant = variants.get_all(None).await.unwrap();
    /// ```
    ///
    /// Read More: https://docs.lemonsqueezy.com/api/variants
    pub async fn get_all(
        &self,
        filters: Option<VariantFilters>,
    ) -> anyhow::Result<VecResponse<Vec<ResponseData<VariantResponse>>>, crate::errors::NetworkError>
    {
        let mut url = "/v1/variants".to_string();

        if filters.is_some() {
            let filter: VariantFilters = filters.unwrap();

            if let Some(store_id) = filter.product_id {
                url.push_str(&format!("?filter[product_id]={}", store_id));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
