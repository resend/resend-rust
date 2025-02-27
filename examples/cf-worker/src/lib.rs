use axum::{routing::get, Router};
use resend_rs::{types::CreateEmailBaseOptions, Resend};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let resend = Resend::new("re_123456789");

    let from = "Acme <onboarding@resend.dev>";
    let to = ["delivered@resend.dev"];
    let subject = "hello world";
    let html = "<p>it works!</p>";

    let email = CreateEmailBaseOptions::new(from, to, subject).with_html(html);

    let _email = resend.emails.send(email).await.unwrap();

    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}
