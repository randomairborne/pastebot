use axum::{
    body::{Bytes, StreamBody},
    extract::{Path, State},
    http::{HeaderMap, HeaderName, HeaderValue, Request},
    middleware::Next,
    response::IntoResponse,
    response::Response,
    routing::get,
};
use futures_core::Stream;
use reqwest::{Client, StatusCode};
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    let http = reqwest::Client::new();
    let app = axum::Router::new()
        .route(
            "/attachment/:channelid/:attachmentid/:filename",
            get(get_file),
        )
        .layer(axum::middleware::from_fn(cors))
        .layer(CompressionLayer::new())
        .with_state(http);
    axum::Server::bind(&([0, 0, 0, 0], 8080).into())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            #[cfg(not(target_family = "unix"))]
            compile_error!("Windows is not supported");
            let mut term_handler =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {},
                _ = term_handler.recv() => {}
            }
            eprintln!("Shutting down..");
        })
        .await
        .unwrap();
}

const CORS_HN: HeaderName = HeaderName::from_static("access-control-allow-origin");

async fn cors<B>(request: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(CORS_HN, HeaderValue::from_static("https://paste.valk.sh"));
    response
}

type StreamItem = Result<Bytes, reqwest::Error>;

#[axum::debug_handler]
async fn get_file(
    Path((channel_id, attachment_id, filename)): Path<(String, String, String)>,
    State(http): State<Client>,
) -> Result<(HeaderMap, StreamBody<impl Stream<Item = StreamItem>>), Error> {
    let mut req = http
        .get(format!(
            "https://cdn.discordapp.com/attachments/{channel_id}/{attachment_id}/{filename}"
        ))
        .send()
        .await?;
    if req.status() == StatusCode::NOT_FOUND || req.status() == StatusCode::FORBIDDEN {
        return Err(Error::NotFound);
    }
    let content_type = req
        .headers_mut()
        .remove("content-type")
        .unwrap_or(HeaderValue::from_static("text/plain; charset=utf-8"));
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), content_type);
    Ok((headers, StreamBody::new(req.bytes_stream())))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("404 file not found")]
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "404 not found").into_response(),
            Self::Reqwest(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("HTTP error: {e}"),
            )
                .into_response(),
        }
    }
}
