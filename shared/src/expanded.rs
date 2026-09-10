use crate::{endpoints::*, types::User};
use macros::params;

#[params(Communities, {CommunityId}, Users, {UserId})]
pub struct TestParameters {
    communit_id: u32,
    user_id: u64,
}

fn xx(x: TestParameters) {
    type MyTy = <CommunityId<EndRoute> as RouteCapture<EndRoute>>::Ty;
    let test: MyTy = x.communit_id;
}
