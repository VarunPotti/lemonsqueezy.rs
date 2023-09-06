use serde::{Deserialize, Serialize};

use crate::constants::{Data, Response, VecResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileResponse {
    pub variant_id: i64,
    pub identifier: String,
    pub name: String,
    pub extension: String,
    pub download_url: String,
    pub size: i64,
    pub size_formatted: Option<String>,
    pub version: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub struct FileFilters {
    pub variant_id: Option<i64>,
}

pub struct Files {
    pub(crate) api: crate::LemonSqueezy,
}

impl Files {
    pub fn build(api: crate::LemonSqueezy) -> Self {
        Self { api }
    }

    pub async fn retrieve(
        &self,
        file_id: usize,
    ) -> anyhow::Result<Response<FileResponse>, crate::errors::NetworkError> {
        let response = self
            .api
            .get::<Response<FileResponse>>(&format!("/v1/files/{}", file_id))
            .await?;

        Ok(response)
    }

    pub async fn get_all(
        &self,
        filters: Option<FileFilters>,
    ) -> anyhow::Result<VecResponse<Vec<Data<FileResponse>>>, crate::errors::NetworkError> {
        let mut url = "/v1/files".to_string();

        //https://api.lemonsqueezy.com/v1/customers?filter[store_id]=11
        if filters.is_some() {
            let filter: FileFilters = filters.unwrap();

            if let Some(store_id) = filter.variant_id {
                url.push_str(&format!("?filter[variant_id]={}", store_id));
            }
        }

        let response = self.api.get(&url).await?;

        Ok(response)
    }
}
