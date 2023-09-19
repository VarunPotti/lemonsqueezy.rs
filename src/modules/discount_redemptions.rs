use serde::{Deserialize, Serialize};

use crate::utils::{Response, ResponseData, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscountRedemptionsResponse {
    pub discount_id: i64,
    pub order_id: i64,
    pub discount_name: String,
    pub discount_code: String,
    pub discount_amount: i64,
    pub discount_amount_type: String,
    pub amount: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub struct DiscountRedemptionsFilters {
    pub order_id: Option<i64>,
    pub discount_id: Option<i64>,
}

pub struct DiscountRedemptions {
    pub(crate) api: crate::LemonSqueezy,
}

impl DiscountRedemptions {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve a discount redemption
    ///
    /// # Arguments
    /// - discount_redemption_id: The ID of the discount redemption to retrieve
    ///
    /// # Returns
    /// - `anyhow::Result<Response<DiscountRedemptionsResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::discount_redemptions::DiscountRedemptions;
    /// let discount_redemptions = DiscountRedemptions::build(lemonsqueezy);
    /// let discount_redemption = discount_redemptions.retrieve(1).await;
    /// ```
    ///
    pub async fn retrieve(
        &self,
        discount_redemption_id: usize,
    ) -> anyhow::Result<Response<DiscountRedemptionsResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<DiscountRedemptionsResponse>>(&format!(
                "/v1/discount-redemptions/{}",
                discount_redemption_id
            ))
            .await?;

        Ok(response)
    }

    /// Retrieve all discount redemptions
    ///
    /// # Arguments
    /// - filters: The discount redemption filters
    ///
    /// # Returns
    /// - `anyhow::Result<VecResponse<Vec<ResponseData<DiscountRedemptionsResponse>>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::discount_redemptions::DiscountRedemptions;
    /// let discount_redemptions = DiscountRedemptions::build(lemonsqueezy);
    /// let discount_redemptions = discount_redemptions.get_all(None).await;
    /// ```
    pub async fn get_all(
        &self,
        filters: Option<DiscountRedemptionsFilters>,
    ) -> anyhow::Result<
        VecResponse<Vec<ResponseData<DiscountRedemptionsResponse>>>,
        crate::errors::NetworkError,
    > {
        let mut url = "/v1/discount-redemptions".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: DiscountRedemptionsFilters = filters.unwrap();

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
}
