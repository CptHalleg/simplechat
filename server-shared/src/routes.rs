use log::info;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use shared::endpoints::{CrudMethod, Endpoint, PathSegment};
use sqlx::{Pool, Postgres};
use warp::Filter;
use warp::http::StatusCode;
use warp::path;

use crate::routes;

pub struct RouteBuilder {
    route: warp::filters::BoxedFilter<()>,

    pool: Pool<Postgres>,
    me_url: String,
}

impl RouteBuilder {
    //pub fn new(pool: Pool<Postgres>, me_url: String) -> Self {,pool, me_url}

    pub fn build(
        self,
    ) -> impl Filter<Extract = (), Error = warp::Rejection> + Clone + Send + Sync + 'static {
        self.route
    }

    pub fn route<Path, In, Out, End, Han, Fut>(
        self,
        _end: End,
        handler: Han,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    where
        Path: PathSegment,
        In: DeserializeOwned + Send + 'static,
        Out: Serialize + Send + 'static,
        End: Endpoint<Input = In, Output = Out, Path = Path>,
        Han: Fn(End::Input, Pool<Postgres>, String) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<End::Output, (StatusCode, String)>> + Send + 'static,
    {
        let method_filter = match End::METHOD {
            CrudMethod::Read => warp::get().boxed(),
            CrudMethod::Create => warp::post().boxed(),
            CrudMethod::Update => warp::put().boxed(),
            CrudMethod::Delete => warp::delete().boxed(),
        };

        self.route
            .and(method_filter)
            .and(warp::body::json())
            .and_then(move |payload: In| {
                let pool = self.pool.clone();
                let me_url = self.me_url.clone();

                let future = handler(payload, pool, me_url);

                async move {
                    match future.await {
                        Ok(response) => Ok::<_, warp::Rejection>(warp::reply::with_status(
                            warp::reply::json(&response),
                            warp::http::StatusCode::OK,
                        )),
                        Err((status_code, error)) => Ok(warp::reply::with_status(
                            warp::reply::json(&serde_json::json!({
                                "error": error
                            })),
                            status_code,
                        )),
                    }
                }
            })
    }
}
