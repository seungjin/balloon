use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Params, Request, Response, Router},
    http_component,
    key_value::{Store},
};
use std::str;

#[http_component]
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get_async("data", api::get_data);
    router.post_async("data", api::post_data);
    router.any_async("/*", api::rest);
    router.handle(req)
}

mod api {
    use serde::Serialize;
    use super::*;

    #[derive(Serialize)]
    struct MyValue {
        value: String,
    }

    pub async fn get_data(req: Request, params: Params) -> Result<impl IntoResponse> {
        let key = req.query();
        let store = Store::open_default()?;
        let value = store.get(key)?;

        match value {
            None => Ok(Response::new(404, "Key not found".to_string())),
            Some(v) => {
                let value = str::from_utf8(&*v).unwrap().to_string();
                let my_value = MyValue { value };
                let json_string = serde_json::to_string(&my_value).unwrap();
                Ok(
                    Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .body(json_string).build()
                )
            },
        }
    }
    pub async fn post_data(req: Request, params: Params) -> Result<impl IntoResponse> {
        let key = req.query();
        let store = Store::open_default()?;
        let data = str::from_utf8(req.body()).unwrap();
        let json: serde_json::Value = serde_json::from_str(data)?;

        match json["value"].as_str() {
            None => return Ok(
                Response::builder().status(400).build()
            ),
            Some(v) => store.set(key, v.as_ref()),
        }?;

        Ok(
            Response::builder().status(201).build()
        )
    }
    pub async fn rest(_req: Request, params: Params) -> Result<impl IntoResponse> {
        Ok(
            Response::builder().status(404).build()
        )
    }
}

