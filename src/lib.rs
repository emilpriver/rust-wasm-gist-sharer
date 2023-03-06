use worker::*;

mod handlers;
mod types;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| {
            let mut headers: http::HeaderMap = Headers::new().into();
            headers.append("Cache-Control", "max-age=2629746".parse().unwrap());

            let rendered = utils::get_web_template();

            return Response::ok(rendered).map(|res| res.with_headers(headers.into()));
        })
        .post_async("/", |req, ctx| async move {
            handlers::create_paste(req, ctx).await
        })
        .get_async(
            "/:id",
            |_, ctx| async move { handlers::get_paste(ctx).await },
        )
        .delete_async(
            "/:id",
            |_, ctx| async move { handlers::delete_paste(ctx).await },
        )
        .run(req, env)
        .await
}
