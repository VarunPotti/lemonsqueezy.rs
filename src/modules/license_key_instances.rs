use serde::{Deserialize, Serialize};

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseKeyInstancesResponse {
    pub license_key_id: i64,
    pub identifier: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct LicenseKeyInstancesFilters {
    pub license_key_id: Option<usize>,
}

pub struct LicenseKeyInstances {
    pub(crate) api: crate::LemonSqueezy,
}

impl LicenseKeyInstances {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve a license key instance
    ///
    /// # Arguments
    /// - license_key_id: The ID of the license key instance to retrieve
    ///
    /// # Returns
    /// - `anyhow::Result<Response<LicenseKeyInstancesResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::license_key_instances::LicenseKeyInstances;
    ///
    /// let license_key_instances = LicenseKeyInstances::build(lemonsqueezy);
    /// let license_key_instance = license_key_instances.retrieve(1).await;
    /// ```
    pub async fn retrieve(
        &self,
        license_key_id: usize,
    ) -> anyhow::Result<Response<LicenseKeyInstancesResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<LicenseKeyInstancesResponse>>(&format!(
                "/v1/license-key-instances/{}",
                license_key_id
            ))
            .await?;

        Ok(response)
    }

    /// Retrieve all license key instances
    ///
    /// # Arguments
    /// - filters: The license key instance filters
    ///
    /// # Returns
    /// - `anyhow::Result<VecResponse<Vec<ResponseData<LicenseKeyInstancesResponse>>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::license_key_instances::LicenseKeyInstances;
    ///
    /// let license_key_instances = LicenseKeyInstances::build(lemonsqueezy);
    /// let license_key_instances = license_key_instances.get_all(None).await.unwrap();
    /// ```
    pub async fn get_all(
        &self,
        filters: Option<LicenseKeyInstancesFilters>,
    ) -> anyhow::Result<
        VecResponse<Vec<ResponseData<LicenseKeyInstancesResponse>>>,
        crate::errors::NetworkError,
    > {
        let mut url = "/v1/license-key-instances".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: LicenseKeyInstancesFilters = filters.unwrap();

            if let Some(license_key_id) = filter.license_key_id {
                url.push_str(&format!("?filter[license_key_id]={}", license_key_id));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
