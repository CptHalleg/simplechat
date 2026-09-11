use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::routing::MethodRouter;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use log::info;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use shared::endpoints::CrudMethod;
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
        info!("registering {} {}", End::METHOD, Params::get_route_string());

        let handler = async move |State(state): State<Context>,
                                  Path(path): Path<Params>,
                                  Json(json): Json<In>|
                    -> Result<Json<Out>, String> {
            info!(
                "recieved request at {} {}",
                End::METHOD,
                Params::get_route_string()
            );
            handler(state, path, json).await.map(|x| Json(x))
        };
        let method_function: MethodRouter<Context> = match End::METHOD {
            CrudMethod::Create => post(handler),
            CrudMethod::Read => get(handler),
            CrudMethod::Update => put(handler),
            CrudMethod::Delete => delete(handler),
        };

        let x = self.router.route(
            &Params::get_route_string(),
            method_function.with_state(self.context.clone()),
        );

        RouteBuilder {
            router: x,
            context: self.context,
        }
    }
}
