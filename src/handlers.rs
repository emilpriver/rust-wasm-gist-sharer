use crate::{types, utils::syntax_highlight_code};
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
        Some(FormEntry::Field(c)) => c,
        None => {
            return Response::from_json(&types::JsonResponse {
                message: "missing 'content' field".to_string(),
            })
            .map(|res| res.with_status(400))
        }
    };

    match code_paste_kv.put(id.as_str(), code)?.execute().await {
        Ok(..) => {
            let _mime_json = "application/json".to_string();
            let accept = match req.headers().get("accept") {
                Ok(Some(value)) => value,
                _ => "".to_string(),
            };

            if accept == _mime_json {
                return Response::from_json(&types::CodePaseResponse {
                    permalink: format!("http://paste.priver.dev/{}", id.as_str()),
                    id: id.to_string(),
                });
            }

            let language = match form.get("language") {
                Some(FormEntry::Field(c)) => format!(".{}", c),
                __ => "".to_string(),
            };

            Response::redirect(
                format!("http://paste.priver.dev/{}{}", id.as_str(), language)
                    .parse()
                    .unwrap(),
            )
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

pub async fn get_paste(ctx: RouteContext<()>) -> Result<Response, Error> {
    let param: Vec<&str> = match ctx.param("id") {
        Some(value) => value.split(".").collect(),
        None => {
            return Response::from_json(&types::JsonResponse {
                message: "missing id".to_string(),
            })
            .map(|res| res.with_status(404));
        }
    };

    let id = param[0];
    console_log!("{}", id);

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
            headers.append("Content-Type", "text/html".parse().unwrap());

            match param.len() > 1 {
                true => {
                    let ext = param[1];

                    let rendered = match syntax_highlight_code(value, ext.to_string()) {
                        Ok(value) => value,
                        Err(error) => {
                            console_log!("{:?}", error);
                            return Response::from_json(&types::JsonResponse {
                                message: "couldn't syntax highlight code".to_string(),
                            })
                            .map(|res| res.with_status(500));
                        }
                    };

                    return Response::ok(rendered).map(|res| res.with_headers(headers.into()));
                }
                false => {
                    return Response::ok(value).map(|res| res.with_headers(headers.into()));
                }
            }
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
    let id = match ctx.param("id") {
        Some(value) => value,
        None => {
            return Response::from_json(&types::JsonResponse {
                message: "missing id".to_string(),
            })
            .map(|res| res.with_status(404));
        }
    };

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
        Ok(..) => Response::from_json(&types::JsonResponse {
            message: "deleted".to_string(),
        })
        .map(|res| res.with_status(200)),
        Err(err) => {
            console_error!("error deleting from KV: {:?}", err.to_string());
            Response::from_json(&types::JsonResponse {
                message: "missing id".to_string(),
            })
            .map(|res| res.with_status(404))
        }
    }
}
