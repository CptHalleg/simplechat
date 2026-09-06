use std::{marker::PhantomData, str::FromStr};

use serde::{Serialize, de::DeserializeOwned};

use crate::payloads::*;

pub enum CrudMethod {
    Create,
    Read,
    Update,
    Delete,
}

pub trait PathSegment {}

pub struct PathEnd {}
impl PathSegment for PathEnd {}

pub trait StaticPath: PathSegment {
    const VALUE: &'static str;
}

trait PathParameter<T, N>: PathSegment
where
    T: FromStr + ToString + Send + 'static,
    N: PathSegment,
{
}

pub struct Users<N>
where
    N: PathSegment,
{
    _marker: PhantomData<N>,
}
impl<N> PathSegment for Users<N> where N: PathSegment {}
impl<N> StaticPath for Users<N>
where
    N: PathSegment,
{
    const VALUE: &str = "users";
}

pub trait Parameter<Path>
where
    Path: PathSegment,
    Self: Sized,
{
    fn to_string(self: Self) -> String;
    fn from_string(string: String) -> Self;
}

pub struct NoParameters {}
impl<T> Parameter<T> for NoParameters
where
    T: PathSegment,
{
    fn to_string(self: NoParameters) -> String {
        String::from("")
    }

    fn from_string(_string: String) -> Self {
        NoParameters {}
    }
}

pub trait Endpoint {
    const METHOD: CrudMethod;

    type Path: PathSegment;
    type Parameters: Parameter<Self::Path>;
    type Input: DeserializeOwned + Send + 'static;
    type Output: Serialize + Send + 'static;
}

pub struct CreateUserProfileEndpoint;
impl Endpoint for CreateUserProfileEndpoint {
    const METHOD: CrudMethod = CrudMethod::Create;

    type Path = Users<PathEnd>;
    type Parameters = NoParameters;
    type Input = InCreateUser;
    type Output = OutCreateUser;
}
