use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{Data, Response, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerResponse {
    pub store_id: i64,
    pub name: String,
    pub email: String,
    pub status: String,
    pub city: Value,
    pub region: Value,
    pub country: String,
    pub total_revenue_currency: i64,
    pub mrr: i64,
    pub status_formatted: String,
    pub country_formatted: String,
    pub total_revenue_currency_formatted: String,
    pub mrr_formatted: String,
    pub created_at: String,
    pub updated_at: String,
    pub test_mode: bool,
}

pub struct CustomerFilters {
    pub store_id: Option<i64>,
    pub email: Option<String>,
}

pub struct Customer {
    pub(crate) api: crate::LemonSqueezy,
}

impl Customer {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        customer_id: usize,
    ) -> anyhow::Result<Response<CustomerResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<CustomerResponse>>(&format!("/v1/customers/{}", customer_id))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<CustomerFilters>,
    ) -> anyhow::Result<VecResponse<Vec<Data<CustomerResponse>>>, crate::errors::NetworkError> {
        let mut url = "/v1/customers".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: CustomerFilters = filters.unwrap();

            let mut added = false;

            if let Some(store_id) = filter.store_id {
                url.push_str(&format!("?filter[store_id]={}", store_id));

                added = true;
            }

            if let Some(email) = filter.email {
                if added {
                    url.push_str(&format!("&filter[email]={}", email));
                } else {
                    url.push_str(&format!("?filter[email]={}", email));
                }
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
