use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use std::result::Result;
use worker::{wasm_bindgen::UnwrapThrowExt, *};

pub async fn create_paste(mut req: Request, ctx: RouteContext<()>) -> Result<Response, Error> {
    let form = req.form_data().await?;

    let code = match form.get("code").unwrap_throw() {
        FormEntry::Field(input) => input.to_string(),
        FormEntry::File(_) => return Response::error("Bad Request", 400),
    };

    let language = match form.get("language").unwrap_throw() {
        FormEntry::Field(input) => input.to_string(),
        FormEntry::File(_) => return Response::error("Bad Request", 400),
    };

    let code_paste_kv = ctx.kv("code_paste")?;

    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    code_paste_kv.put(format!("{:?}.{:?}", id, language).as_str(), code)?;

    Response::error("Bad Request", 400)
}
