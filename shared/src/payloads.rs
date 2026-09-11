use macros::PayloadIn;
use macros::PayloadOut;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::types::*;

pub trait PayloadIn: Serialize + DeserializeOwned + Debug + Clone + Send + 'static {}
pub trait PayloadOut: Serialize + DeserializeOwned + Debug + Clone + Send + 'static {}

#[derive(PayloadIn, PayloadOut, Serialize, Deserialize, Debug, Clone)]
pub struct NoPayload;

#[derive(PayloadIn, Serialize, Deserialize, Debug, Clone)]
pub struct InCreateUser {
    pub name: String,
}

#[derive(PayloadOut, Deserialize, Serialize, Debug, Clone)]
pub struct OutCreateUser {
    pub user: User,
}

#[derive(PayloadOut, Serialize, Deserialize, Debug, Clone)]
pub struct OutGetUser {
    pub user: User,
}
