use crate::parameters::Parameters;
use crate::route::RouteCapture;
use crate::route::RouteConstant;
use crate::route::{Communities, CommunityId, EndRoute, UserId, Users};
use macros::params;
use serde::Deserialize;

#[derive(Deserialize)]
#[params(Communities, {CommunityId}, Users, {UserId})]
pub struct TestParameters {
    communit_id: u32,
    user_id: u64,
}

fn xx(x: TestParameters) {
    type MyTy = <CommunityId<EndRoute> as RouteCapture<EndRoute>>::Ty;
    let test: MyTy = x.communit_id;
}
