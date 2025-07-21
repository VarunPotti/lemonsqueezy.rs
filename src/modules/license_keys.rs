use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::header::{HeaderMap, HeaderValue};
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
    pub activation_limit: Option<i64>,
    pub instances_count: i64,
    pub disabled: bool,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseActivationMeta {
	pub store_id: i64,
	pub order_id: i64,
	pub order_item_id: i64,
	pub product_id: i64,
	pub product_name: String,
	pub variant_id: i64,
	pub variant_name: String,
	pub customer_id: i64,
    pub customer_name: String,
    pub customer_email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseActivationInstance {
	pub id: String, 
	pub name: String,
	pub created_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseActivationKey {
	pub id: i64,
	pub status: String,
	pub key: String,
	pub activation_limit: Option<i64>,
	pub activation_usage: i64,
	pub created_at: String,
	pub expires_at: Option<String>,
}	

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseActivationResponse {
	pub activated: bool,
	pub error: Option<String>,
	pub license_key: Option<LicenseActivationKey>,
	pub instance: Option<LicenseActivationInstance>,
	pub meta: Option<LicenseActivationMeta>,
}

pub struct LicenseKey {
    pub(crate) api: crate::LemonSqueezy,
}

impl LicenseKey {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Activate a license key
    ///
    /// # Arguments
    /// - license_key: The license key to activate
    /// - instance_name: A label for the new instance to identify it in Lemon Squeezy.
    ///
    /// # Returns
    /// - `anyhow::Result<LicenseActivationResponse, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::license_keys::LicenseKey;
    /// let license_keys = LicenseKey::build(lemonsqueezy);
    /// let activation_response = license_keys.activate("38b1460a-5104-4067-a91d-77b872934d51", "Test").await;
    /// ```
	pub async fn activate (
		&self, 
		license_key: &str,
        instance_name: &str,
	) -> anyhow::Result<LicenseActivationResponse, crate::errors::NetworkError> {

        let mut headers = HeaderMap::new();
        headers.append(
            "Accept",
            HeaderValue::from_str("Accept: application/json").unwrap(),
        );

        let form = [
            ("license_key", license_key),
            ("instance_name", instance_name),
        ];

		let response = self
            .api
            .post_form::<LicenseActivationResponse>("/v1/licenses/activate", headers, &form)
            .await?;

        Ok(response)
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
