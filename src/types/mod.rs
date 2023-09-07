use serde::{Deserialize, Serialize};
pub mod discount;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data<T> {
    pub data: T,
}
