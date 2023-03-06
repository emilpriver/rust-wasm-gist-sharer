use crate::{
    types,
    utils::{syntax_highlight_code},
};
use rand::{distributions::Alphanumeric, Rng};
use std::result::Result;

use worker::*;

pub async fn create_paste(mut req: Request, ctx: RouteContext<()>) -> Result<Response, Error> {
    let form = req.form_data().await?;

    let code_paste_kv = ctx.kv("code_paste")?;

    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let code = match form.get("content") {
        Some(FormEntry::File(file)) => {
            let bytes = file.bytes().await?;

            match String::from_utf8(bytes) {
                Ok(v) => v,
                Err(e) => {
                    console_error!("Invalid UTF-8 sequence: {}", e);

                    return Response::from_json(&types::JsonResponse {
                        message: "couldn't parse file".to_string(),
                    })
                    .map(|res| res.with_status(400));
                }
            }
        }
        Some(FormEntry::Field(c)) => c.to_string(),
        None => {
            return Response::from_json(&types::JsonResponse {
                message: "missing 'content' field".to_string(),
            })
            .map(|res| res.with_status(400))
        }
    };

    let language = match form.get("language") {
        Some(FormEntry::File(..)) => {
            return Response::from_json(&types::JsonResponse {
                message: "expected 'language' to be a string".to_string(),
            })
            .map(|res| res.with_status(400));
        }
        Some(FormEntry::Field(c)) => c.to_string().replace(" ", "-"),
        None => {
            return Response::from_json(&types::JsonResponse {
                message: "missing 'language' field".to_string(),
            })
            .map(|res| res.with_status(400))
        }
    };

    let code_paste_id = format!("{}.{}", id.to_string(), language);

    match code_paste_kv
        .put(code_paste_id.as_str(), code)?
        .execute()
        .await
    {
        Ok(..) => {
            let _mime_json = "application/json".to_string();
            match req.headers().get("accept").unwrap() {
                Some(_mime_json) => Response::from_json(&types::CodePaseResponse {
                    permalink: format!("http://paste.priver.dev/{}", code_paste_id.as_str()),
                    id: code_paste_id.to_string(),
                }),
                _ => Response::redirect(
                    format!("http://paste.priver.dev/{}", code_paste_id.as_str())
                        .parse()
                        .unwrap(),
                ),
            }
        }
        Err(err) => {
            console_error!("error posting data to KV: {:?}", err.to_string());
            Response::from_json(&types::JsonResponse {
                message: "couldn't add pase to database".to_string(),
            })
            .map(|res| res.with_status(400))
        }
    }
}

pub async fn get_paste(ctx: RouteContext<()>, use_raw_format: bool) -> Result<Response, Error> {
    if let None = ctx.param("id") {
        return Response::from_json(&types::JsonResponse {
            message: "missing id".to_string(),
        })
        .map(|res| res.with_status(404));
    };

    let id = ctx.param("id").unwrap();

    let code_paste_kv = match ctx.kv("code_paste") {
        Ok(value) => value,
        Err(err) => {
            console_error!("error reading kv: {:?}", err.to_string());
            return Response::from_json(&types::JsonResponse {
                message: "missing id".to_string(),
            })
            .map(|res| res.with_status(404));
        }
    };

    match code_paste_kv.get(id).text().await {
        Ok(Some(value)) => {
            let mut headers: http::HeaderMap = Headers::new().into();
            headers.append("Cache-Control", "max-age=2629746".parse().unwrap());

            if use_raw_format {
                return Response::ok(value).map(|res| res.with_headers(headers.into()));
            }

            let (_, language) = id.split_once(".").unwrap();

            let rendered = syntax_highlight_code(value.clone(), language.to_string());

            Response::ok(rendered).map(|res| res.with_headers(headers.into()))
        }
        Ok(None) => Response::from_json(&types::JsonResponse {
            message: "missing id".to_string(),
        })
        .map(|res| res.with_status(404)),
        Err(err) => {
            console_error!("error reading kv data: {:?}", err.to_string());
            Response::from_json(&types::JsonResponse {
                message: "missing id".to_string(),
            })
            .map(|res| res.with_status(404))
        }
    }
}

pub async fn delete_paste(ctx: RouteContext<()>) -> Result<Response, Error> {
    if let None = ctx.param("id") {
        return Response::from_json(&types::JsonResponse {
            message: "missing id".to_string(),
        })
        .map(|res| res.with_status(404));
    };

    let id = ctx.param("id").unwrap();

    let code_paste_kv = match ctx.kv("code_paste") {
        Ok(value) => value,
        Err(err) => {
            console_error!("error reading kv: {:?}", err.to_string());
            return Response::from_json(&types::JsonResponse {
                message: "missing id".to_string(),
            })
            .map(|res| res.with_status(404));
        }
    };

    match code_paste_kv.delete(id).await {
        Ok(..) => {
            return Response::from_json(&types::JsonResponse {
                message: "deleted".to_string(),
            })
            .map(|res| res.with_status(200))
        }
        Err(err) => {
            console_error!("error deleting from KV: {:?}", err.to_string());
            return Response::from_json(&types::JsonResponse {
                message: "missing id".to_string(),
            })
            .map(|res| res.with_status(404));
        }
    }
}
