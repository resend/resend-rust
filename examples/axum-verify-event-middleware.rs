// Used:
// axum = "0.7.9"
// svix = "1.42.0"
// http-body-util = "0.1.2"

use axum::{
    Router,
    body::Body,
    extract::Request,
    http,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
};

use http::{HeaderMap, StatusCode};

use http_body_util::BodyExt;
use resend_rs::events::try_parse_event;
use svix::webhooks::Webhook;

#[allow(clippy::unwrap_used)]
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(handler))
        .layer(middleware::from_fn(verify_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler(data: String) -> StatusCode {
    // Put your normal endpoint code here
    let event = try_parse_event(&data);
    println!("{event:?}");

    StatusCode::OK
}

// Mostly taken from https://github.com/tokio-rs/axum/blob/main/examples/consume-body-in-extractor-or-middleware/src/main.rs
async fn verify_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let (parts, body) = request.into_parts();

    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    let secret = "<YOUR RESEND SIGNING SECRET HERE>".to_string();
    let wh = Webhook::new(&secret).expect("Invalid secret");
    wh.verify(&bytes, &headers)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;

    let request = Request::from_parts(parts, Body::from(bytes));

    Ok(next.run(request).await)
}
