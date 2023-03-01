use crate::types;
use rand::{distributions::Alphanumeric, Rng};
use std::result::Result;
use worker::*;

pub async fn create_paste(mut req: Request, ctx: RouteContext<()>) -> Result<Response, Error> {
    let json = req.json::<types::CodePasteInput>().await?;

    let code_paste_kv = ctx.kv("code_paste")?;

    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let language = json.language.to_string();
    let code_paste_id = format!("{}.{}", id.to_string(), language);

    match code_paste_kv
        .put(code_paste_id.as_str(), json.code.to_string())?
        .execute()
        .await
    {
        Ok(..) => Response::redirect(
            format!("http://localhost:8787/{}", code_paste_id.as_str())
                .parse()
                .unwrap(),
        ),
        Err(err) => {
            console_error!("error posting data to KV: {:?}", err.to_string());
            Response::from_json(&types::JsonResponse {
                message: "couldn't add pase to database".to_string(),
            })
        }
    }
}

pub async fn get_paste(mut req: Request, ctx: RouteContext<()>) -> Result<Response, Error> {
    if let None = ctx.param("id") {
        return Response::from_json(&types::JsonResponse {
            message: "missing id".to_string(),
        });
    };

    let id = ctx.param("id").unwrap();

    let code_paste_kv = ctx.kv("code_paste")?;
    match code_paste_kv.get(id).text().await {
        Ok(value) => {
            let mut headers: http::HeaderMap = Headers::new().into();
            headers.append("Cache-Control", "max-age=2629746".parse().unwrap());

            Response::ok(value.unwrap()).map(|res| res.with_headers(headers.into()))
        }
        Err(err) => {
            console_error!("error reading kv data: {:?}", err.to_string());
            Response::from_json(&types::JsonResponse {
                message: "missing id".to_string(),
            })
        }
    }
}
