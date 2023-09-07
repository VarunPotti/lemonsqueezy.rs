use serde::{Deserialize, Serialize};

use super::Data;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDiscountRelationships {
    pub store: Data<CreateDiscountRelationshipsStore>,
    pub variants: Data<Vec<CreateDiscountRelationshipsVariants>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDiscountRelationshipsStore {
    pub r#type: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDiscountRelationshipsVariants {
    pub r#type: String,
    pub id: String,
}
