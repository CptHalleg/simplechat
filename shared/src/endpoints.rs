use std::{marker::PhantomData, str::FromStr};

use macros::params;
use macros::route_capture;
use macros::route_constant;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::payloads::*;

pub trait Route {}
pub trait RouteSegment<N>: Route
where
    N: Route,
{
}
pub trait RouteCapture<N>: RouteSegment<N>
where
    N: Route,
{
    const NAME: &'static str;
    type Ty: Serialize + DeserializeOwned;
}
pub trait RouteConstant<N>: RouteSegment<N>
where
    N: Route,
{
    const VALUE: &'static str;
}

pub struct EndRoute;
impl Route for EndRoute {}

route_constant!(Users);
route_constant!(Communities);

route_capture!(UserId: u64);
route_capture!(CommunityId: u32);

pub enum PathSegmentKind {
    Const(String),
    String,
    U32,
}

pub enum CrudMethod {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Deserialize, Clone)]
#[params(Users)]
#[params()]
pub struct NoParams;

#[derive(Deserialize, Clone)]
#[params(Users, {UserId})]
pub struct GetUserParams;

pub trait Parameters<R>
where
    R: Route,
{
    fn get_route_string() -> String;
}

pub trait Endpoint {
    const METHOD: CrudMethod;

    type Route: Route;
    type Parameters: Parameters<Self::Route>;
    type Input: DeserializeOwned + Send + 'static;
    type Output: Serialize + Send + 'static;
}

pub struct CreateUserProfileEndpoint;
impl Endpoint for CreateUserProfileEndpoint {
    const METHOD: CrudMethod = CrudMethod::Create;

    type Route = Users<EndRoute>;
    type Parameters = NoParams;
    type Input = InCreateUser;
    type Output = OutCreateUser;
}

pub struct GetUserProfileEndpoint;
impl Endpoint for GetUserProfileEndpoint {
    const METHOD: CrudMethod = CrudMethod::Read;

    type Route = Users<UserId<EndRoute>>;
    type Parameters = GetUserParams;
    type Input = NoPayload;
    type Output = OutGetUser;
}

pub struct TestProfileEndpoint;
impl Endpoint for TestProfileEndpoint {
    const METHOD: CrudMethod = CrudMethod::Read;

    type Route = EndRoute;
    type Parameters = NoParams;
    type Input = NoPayload;
    type Output = NoPayload;
}
