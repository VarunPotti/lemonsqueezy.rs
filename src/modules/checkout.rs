pub use crate::types::checkout::*;

use serde_json::json;

use crate::utils::{Response, ResponseData, VecResponse};

pub struct WebhookRedemptionsFilters {
    pub store_id: Option<i64>,
}

pub struct Checkout {
    pub(crate) api: crate::LemonSqueezy,
}

impl Checkout {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    /// Retrieve a checkout
    ///
    /// # Arguments
    /// - checkout_id: The ID of the checkout to retrieve
    ///
    /// # Returns
    /// - `anyhow::Result<Response<CheckoutResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::checkout::Checkout;
    /// let checkout = Checkout::build(lemonsqueezy);
    /// let checkout = checkout.retrieve(1).await;
    /// ```
    pub async fn retrieve(
        &self,
        checkout_id: String,
    ) -> anyhow::Result<Response<CheckoutResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<CheckoutResponse>>(&format!("/v1/checkouts/{}", checkout_id))
            .await?;

        Ok(response)
    }

    /// Retrieve all checkouts
    ///
    /// # Arguments
    /// - filters: The checkout filters
    ///
    /// # Returns
    /// - `anyhow::Result<VecResponse<Vec<ResponseData<CheckoutResponse>>>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::checkout::Checkout;
    /// let checkout = Checkout::build(lemonsqueezy);
    /// let checkout = checkout.get_all(None).await;
    /// ```
    pub async fn get_all(
        &self,
        filters: Option<WebhookRedemptionsFilters>,
    ) -> anyhow::Result<VecResponse<Vec<ResponseData<CheckoutResponse>>>, crate::errors::NetworkError>
    {
        let mut url = "/v1/checkouts".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: WebhookRedemptionsFilters = filters.unwrap();

            if let Some(store_id) = filter.store_id {
                url.push_str(&format!("?filter[store_id]={}", store_id));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }

    /// Create a checkout
    ///
    /// # Arguments
    /// - data: The checkout data
    ///
    /// # Returns
    /// - `anyhow::Result<Response<CheckoutResponse>, crate::errors::NetworkError>` object
    ///
    /// # Example
    /// ```
    /// use lemonsqueezy::checkout::Checkout;
    /// use lemonsqueezy::types::checkout::*;
    ///
    /// let checkout = Checkout::build(lemonsqueezy);
    // let checkout = checkout
    //     .create(CreateCheckout {
    //         r#type: String::from("checkouts"),
    //         attributes: CreateCheckoutAttributes {
    //             ..Default::default()
    //         },
    //         relationships: Some(CreateCheckoutRelationships {
    //             store: lemonsqueezy::types::Data {
    //                 data: CreateCheckoutRelationShipData {
    //                     r#type: String::from("stores"),
    //                     id: String::from("//store id"),
    //                 },
    //             },
    //             variant: lemonsqueezy::types::Data {
    //                 data: CreateCheckoutRelationShipData {
    //                     r#type: String::from("variants"),
    //                     id: String::from("// variant id"),
    //                 },
    //             },
    //         }),
    //     }).await;
    /// ```
    pub async fn create(
        &self,
        data: CreateCheckout,
    ) -> anyhow::Result<Response<CheckoutResponse>, crate::errors::NetworkError> {
        let data = json!({
            "data": data,
        });

        let response = self.api.post("/v1/checkouts", data).await?;

        Ok(response)
    }
}
