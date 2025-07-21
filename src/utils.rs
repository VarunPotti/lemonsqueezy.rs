use std::collections::HashMap;

pub const API_URL: &str = "https://api.lemonsqueezy.com";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JsonAPI {
    version: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Links {
    first: Option<String>,
    last: Option<String>,

    next: Option<String>,
    prev: Option<String>,

    #[serde(rename = "self")]
    _self: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Response<T> {
    jsonapi: Option<JsonAPI>,
    links: Option<Links>,
    data: Option<ResponseData<T>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct VecResponse<T> {
    jsonapi: Option<JsonAPI>,
    links: Option<Links>,
    data: T,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ResponseData<T> {
    pub r#type: String,
    pub id: String,
    pub relationships: Option<HashMap<String, HashMap<String, RelationShip>>>,
    pub attributes: T,
    pub links: Option<Links>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RelationShip {
    related: Option<String>,
    #[serde(rename = "self")]
    _self: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ResponseMeta<T> {
    jsonapi: Option<JsonAPI>,
    meta: T,
}


impl<T> Response<T> {

    /// Retrieves a reference to the inner response data, if it exists.
    ///
    /// This function attempts to extract the inner data from the `data` field.
    /// If the `data` contains a value and its `attributes` field exists, a reference to it is returned.
    /// If either `data` is `None` or its `attributes` field is `None`, it returns `None`.
    pub fn get_inner_response (&self) -> Option<&T> {
        self.data.as_ref().and_then(|d| Some (&d.attributes))
    }
}