use macros::{route_capture, route_constant};
use serde::{Serialize, de::DeserializeOwned};

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
