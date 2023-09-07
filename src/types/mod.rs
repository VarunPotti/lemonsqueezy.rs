use serde::{Deserialize, Serialize};

pub mod checkout;
pub mod discount;
pub mod webhook;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data<T> {
    pub data: T,
}
