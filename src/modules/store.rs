use serde::{Deserialize, Serialize};

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoreResponse {
    pub name: String,
    pub slug: String,
    pub domain: String,
    pub url: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub plan: String,
    pub country: String,
    #[serde(rename = "country_nicename")]
    pub country_nicename: String,
    pub currency: String,
    #[serde(rename = "total_sales")]
    pub total_sales: i64,
    #[serde(rename = "total_revenue")]
    pub total_revenue: i64,
    #[serde(rename = "thirty_day_sales")]
    pub thirty_day_sales: i64,
    #[serde(rename = "thirty_day_revenue")]
    pub thirty_day_revenue: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub struct Store {
    pub(crate) api: crate::LemonSqueezy,
}

impl Store {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve the store
    ///
    /// # Arguments
    /// * `store_id` - The store id
    ///
    /// # Returns
    /// `Result<Response<StoreResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```rust
    /// use lemonsqueezy::user::Store;
    /// let store = Store::build(lemonsqueezy);
    /// let store = store.retrieve(123).await.unwrap();
    /// ```
    ///
    /// Read More: https://docs.lemonsqueezy.com/api/stores#retrieve-a-store
    pub async fn retrieve(
        &self,
        store_id: usize,
    ) -> anyhow::Result<Response<StoreResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<StoreResponse>>(&format!("/v1/stores/{}", store_id))
            .await?;

        Ok(response)
    }

    /// Retrieve all the stores
    ///
    /// # Returns
    /// `Result<VecResponse<Vec<ResponseData<StoreResponse>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```rust
    /// use lemonsqueezy::user::Store;
    /// let store = Store::build(lemonsqueezy);
    /// let store = store.get_all().await.unwrap();
    /// ```
    ///
    /// Read More: https://docs.lemonsqueezy.com/api/stores#retrieve-a-store
    pub async fn get_all(
        &self,
    ) -> anyhow::Result<VecResponse<Vec<ResponseData<StoreResponse>>>, crate::errors::NetworkError>
    {
        let response = self.api.get("/v1/stores").await.unwrap();

        Ok(response)
    }
}
