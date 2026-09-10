use serde::Deserialize;
use serde::Serialize;

use crate::types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoPayload;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InCreateUser {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutCreateUser {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutGetUser {
    pub user: User,
}
