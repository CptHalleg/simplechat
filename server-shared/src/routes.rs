use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::routing::get;
use log::info;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use shared::endpoints::Endpoint;
use shared::endpoints::Parameters;
use shared::endpoints::Route;
use sqlx::{Pool, Postgres};
use warp::http::StatusCode;

#[derive(Clone)]
pub struct Context {
    pub pool: Pool<Postgres>,
    pub me_url: String,
}

pub struct RouteBuilder {
    router: Router<()>,

    context: Context,
}

impl RouteBuilder {
    pub fn new(pool: Pool<Postgres>, me_url: String) -> Self {
        Self {
            router: Router::new(),
            context: Context { pool, me_url },
        }
    }

    pub fn build(self) -> Router<()> {
        self.router
    }

    pub fn route<Rout, Params, In, Out, End, Han, Fut>(self, _end: End, handler: Han) -> Self
    where
        Rout: Route,
        Params: Parameters<Rout> + DeserializeOwned + Send + 'static,
        In: DeserializeOwned + Send + 'static,
        Out: Serialize + Send + 'static,
        End: Endpoint<Input = In, Output = Out, Parameters = Params, Route = Rout>,
        Han: Fn(Context, Params, In) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = Result<Out, String>> + Send + 'static,
    {
        info!("registering request {}", Params::get_route_string());
        let x = self.router.route(
            &Params::get_route_string(),
            get(
                async move |State(state): State<Context>,
                            Path(path): Path<Params>,
                            Json(json): Json<In>|
                            -> Result<Json<Out>, String> {
                    info!("recieved request {}", Params::get_route_string());
                    handler(state, path, json).await.map(|x| Json(x))
                },
            )
            .with_state(self.context.clone()),
        );

        RouteBuilder {
            router: x,
            context: self.context,
        }
    }
}
