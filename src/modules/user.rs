use serde::{Deserialize, Serialize};

use crate::utils::Response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    pub name: String,
    pub email: String,
    pub color: String,
    pub avatar_url: String,
    pub has_custom_avatar: bool,

    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub struct User {
    pub(crate) api: crate::LemonSqueezy,
}

impl User {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
    ) -> anyhow::Result<Response<UserResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<UserResponse>>("/v1/users/me")
            .await?;

        Ok(response)
    }
}
