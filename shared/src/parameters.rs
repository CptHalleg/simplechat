use crate::route::{Route, Users};
use macros::params;
use serde::{Deserialize, de::DeserializeOwned};

use crate::route::RouteCapture;
use crate::route::RouteConstant;
use crate::route::{EndRoute, UserId};

pub trait Parameters<R>: DeserializeOwned + Send + 'static
where
    R: Route,
{
    fn get_route_string() -> String;
}

#[derive(Deserialize, Clone)]
#[params(Users)]
#[params()]
pub struct NoParams;

#[derive(Deserialize, Clone)]
#[params(Users, {UserId})]
pub struct GetUserParams;
