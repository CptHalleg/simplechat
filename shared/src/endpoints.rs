use std::fmt::Display;

use crate::parameters::GetUserParams;
use crate::parameters::NoParams;
use crate::parameters::Parameters;
use crate::payloads::*;
use crate::route::EndRoute;
use crate::route::Route;
use crate::route::UserId;
use crate::route::Users;

pub enum CrudMethod {
    Create,
    Read,
    Update,
    Delete,
}
impl Display for CrudMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrudMethod::Create => f.write_str("POST"),
            CrudMethod::Read => f.write_str("GET"),
            CrudMethod::Update => f.write_str("PUT"),
            CrudMethod::Delete => f.write_str("DELETE"),
        }
    }
}
pub trait Endpoint {
    const METHOD: CrudMethod;

    type Route: Route;
    type Parameters: Parameters<Self::Route>;
    type Input: PayloadIn;
    type Output: PayloadOut;
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
