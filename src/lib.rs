use errors::NetworkError;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method, Request,
};

pub(crate) mod errors;
pub mod modules;
pub mod utils;
pub use crate::modules::*;
pub mod types;

#[derive(Clone, Debug)]
pub struct LemonSqueezy {
    client: reqwest::Client,
    headers: HeaderMap,
}

impl LemonSqueezy {
    pub fn new(key: String) -> Self {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();

        headers.append(
            "Accept",
            HeaderValue::from_str("application/vnd.api+json").unwrap(),
        );
        headers.append(
            "Content-Type",
            HeaderValue::from_str("application/vnd.api+json").unwrap(),
        );
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", key)).unwrap(),
        );

        Self { client, headers }
    }

    /// Get a resource from the LemonSqueezy API
    ///
    /// ### Arguments
    /// url - The URL to send a `GET` request to
    ///
    /// ### Returns
    /// A `Result` containing either the response body as Generic type T or a `NetworkError`
    ///
    /// ### Example
    /// ```rust
    /// use lemonsqueezy::LemonSqueezy;
    ///
    /// let api = LemonSqueezy::new();
    /// api.get::<serde_json::Value>("/v1/users/me").await.unwrap();
    /// ```
    pub async fn get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        url: &str,
    ) -> anyhow::Result<T, NetworkError> {
        let url = format!("{}{}", utils::API_URL, url);

        let response = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?;

        let json = response.json().await?;

        Ok(json)
    }

    /// Post a form resource to the LemonSqueezy API
    ///
    /// ### Arguments
    /// url - The URL to send a `Post` request to
    /// headers - Custom headers to send with the request
    /// form - The form data to send with the request (list of key-value pairs)
    ///
    /// ### Returns
    /// A `Result` containing either the response body as Generic type T or a `NetworkError`
    pub async fn post_form<V: for<'de> serde::Deserialize<'de>> (
        &self,
        url: &str,
        headers: HeaderMap,
        form: &[(&str, &str)],
    ) -> anyhow::Result<V, NetworkError> {
        let url = format!("{}{}", utils::API_URL, url);

        let response = self
            .client
            .post(url)
            .headers(headers)
            .form(form)
            .send()
            .await?;

        let response = response.json().await?;

        Ok(response)
    }

    /// Post a resource to the LemonSqueezy API
    ///
    /// ### Arguments
    /// url - The URL to send a `Post` request to
    /// body - The body to send with the request
    ///
    /// ### Returns
    /// A `Result` containing either the response body as Generic type T or a `NetworkError`
    ///
    /// ### Example
    /// ```rust
    /// use lemonsqueezy::LemonSqueezy;
    ///
    /// let api = LemonSqueezy::new();
    /// api.post::<serde_json::Value>("/v1/usage-records").await.unwrap();
    /// ```
    pub async fn post<V: for<'de> serde::Deserialize<'de>, T: serde::ser::Serialize>(
        &self,
        url: &str,
        body: T,
    ) -> anyhow::Result<V, NetworkError> {
        let url = format!("{}{}", utils::API_URL, url);

        let response = self
            .client
            .post(url)
            .json(&body)
            .headers(self.headers.clone())
            .send()
            .await?;

        let response = response.json().await?;

        Ok(response)
    }

    /// Delete a resource from the LemonSqueezy API
    ///
    /// ### Arguments
    /// url - The URL to send a `Delete` request to
    ///
    /// ### Returns
    /// A `Result` containing either the response body as Generic type T or a `NetworkError`
    ///
    /// ### Example
    /// ```rust
    /// use lemonsqueezy::LemonSqueezy;
    ///
    /// let api = LemonSqueezy::new();
    /// api.delete::<serde_json::Value>("/v1/discounts/1").await.unwrap();
    /// ```
    pub async fn delete<V: for<'de> serde::Deserialize<'de>>(
        &self,
        url: &str,
    ) -> anyhow::Result<V, NetworkError> {
        let url = format!("{}{}", utils::API_URL, url);

        let response = self
            .client
            .delete(url)
            .headers(self.headers.clone())
            .send()
            .await?;

        let response = response.json().await?;

        Ok(response)
    }

    /// Update a resource from the LemonSqueezy API
    ///
    /// ### Arguments
    /// url - The URL to send a `Patch` request to
    ///
    /// ### Returns
    /// A `Result` containing either the response body as Generic type T or a `NetworkError`
    ///
    /// ### Example
    /// ```rust
    /// use lemonsqueezy::LemonSqueezy;
    ///
    /// let api = LemonSqueezy::new();
    /// api.patch::<serde_json::Value>("/v1/subscriptions/:id").await.unwrap();
    /// ```
    pub async fn patch<T: Into<reqwest::Body>>(
        &self,
        url: &str,
        body: T,
    ) -> anyhow::Result<serde_json::Value, NetworkError> {
        let url = format!("{}{}", utils::API_URL, url);

        let mut request = Request::new(Method::PATCH, url.parse().unwrap());
        request.headers_mut().extend(self.headers.clone());
        request.body_mut().replace(body.into());

        let response = self.client.execute(request).await?;

        let response = response.json().await?;

        Ok(response)
    }
}
