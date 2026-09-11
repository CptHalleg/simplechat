use crate::types::User;

trait Path {}
trait PathSegment: Path {
    type Prev: Path;
    fn get_prev(&self) -> &Self::Prev;
}
trait PathConstant: PathSegment {
    const VALUE: &'static str;
    fn new(prev: Self::Prev) -> Self;
}
trait PathCapture: PathSegment {
    type Type;
    const NAME: &'static str;
    fn new(value: Self::Type, prev: Self::Prev) -> Self;
}

struct PathRoot;
impl Path for PathRoot {}

struct Users {
    prev: PathRoot,
}
impl Path for Users {}
impl PathSegment for Users {
    type Prev = PathRoot;
    fn get_prev(&self) -> &PathRoot {
        &self.prev
    }
}
impl PathConstant for Users {
    const VALUE: &'static str = "Users";

    fn new(prev: PathRoot) -> Self {
        Self { prev }
    }
}

struct UserId {
    prev: Users,
    value: u64,
}
impl Path for UserId {}
impl PathSegment for UserId {
    type Prev = Users;
    fn get_prev(&self) -> &Users {
        &self.prev
    }
}
impl PathCapture for UserId {
    type Type = u64;

    const NAME: &'static str = "user_id";

    fn new(value: u64, prev: Users) -> Self {
        Self { value, prev }
    }
}

fn test() {
    let x: UserId = UserId::new(23, Users::new(PathRoot));
}
