use serde::{Deserialize, Serialize};

use super::Data;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhook {
    pub r#type: String,
    pub attributes: CreateWebhookAttributes,
    pub relationships: Option<CreateWebhookRelationships>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookAttributes {
    pub url: String,

    pub events: Vec<String>,

    pub secret: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookRelationships {
    pub store: Data<CreateWebhookRelationShipData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookRelationShipData {
    pub r#type: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhook {
    pub r#type: String,
    pub id: String,
    pub attributes: UpdateWebhookAttributes,
    pub relationships: Option<UpdateWebhookRelationships>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhookAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhookRelationships {
    pub store: Data<UpdateWebhookRelationShipData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhookRelationShipData {
    pub r#type: String,
    pub id: String,
}
