use log::info;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    endpoints::{CrudMethod, Endpoint, Parameters, Route},
    request::Error::{Invalid, NoConnection},
};

pub enum Error {
    NoConnection,
    Invalid,
}

pub async fn request<Rout, Params, In, Out, End>(
    _end: End,
    client: Client,
    input: In,
) -> Result<Out, Error>
where
    Rout: Route,
    Params: Parameters<Rout> + DeserializeOwned + Send + 'static,
    In: Serialize + Send + 'static,
    Out: DeserializeOwned + Send + 'static,
    End: Endpoint<Input = In, Output = Out, Parameters = Params, Route = Rout>,
{
    let url = format!("http://localhost:3000{}", Params::get_route_string());
    info!("sending request to {} {}...", End::METHOD, url.clone());
    let method_function = match End::METHOD {
        CrudMethod::Create => Client::post,
        CrudMethod::Read => Client::get,
        CrudMethod::Update => Client::put,
        CrudMethod::Delete => Client::delete,
    };
    let response = method_function(&client, url.clone())
        .json(&input)
        .send()
        .await;

    info!("{} {} response: {:#?}", End::METHOD, url.clone(), response);
    match response {
        Ok(val) => match val.json::<Out>().await {
            Ok(val) => Ok(val),
            Err(_) => Err(Invalid),
        },
        Err(val) => match val.is_timeout() {
            true => Err(NoConnection),
            false => Err(Invalid),
        },
    }
}
