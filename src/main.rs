use askama::Template;
use askama_web::WebTemplate;
use poem::{IntoResponse, Response, Route, Server, handler, EndpointExt, listener::TcpListener, middleware::Tracing};
use poem::web::Query;
use poem::{get, post};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use url::Url;
use tracing::info;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct IndexTemplate {
    mf2rust_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct QueryParams {
    url: Url,
    html: Option<String>,
}

#[handler]
async fn parse_handler(Query(query): Query<QueryParams>) -> impl IntoResponse {
    let resp = match query.html {
        None => {
            let client = reqwest::Client::new();
            let response_result = client.get(query.url.as_str()).send().await;
            let body = match response_result {
                Ok(r) => r.text().await.unwrap_or_default(),
                Err(_) => String::default(),
            };
            mf2::from_html(&body, &query.url)
        }
        Some(html) => mf2::from_html(&html, &query.url),
    };

    let doc = resp.unwrap_or_default();
    let mut json_val = serde_json::to_value(&doc).unwrap_or_default();
    
    if let Some(obj) = json_val.as_object_mut() {
        obj.insert("debug".to_string(), json!({
            "package": "https://crates.io/crates/microformats2",
            "version": "0.1.0",
            "note": [
                "This output was generated from microformats2 crate available at https://gitlab.com/maxburon/microformats-parser.",
                "Please file any issues with the parser at https://gitlab.com/maxburon/microformats-parser/issues"
            ]
        }));
    }

    Response::builder()
        .header("content-type", "application/json; utf-8")
        .body(serde_json::to_string_pretty(&json_val).unwrap_or_default())
}

#[handler]
async fn index_handler() -> impl IntoResponse {
    IndexTemplate {
        mf2rust_version: "0.16.1".to_string(),
    }
}

#[handler]
async fn catch_all() -> impl IntoResponse {
    IndexTemplate {
        mf2rust_version: "0.16.1".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap_or(8000);
    
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .unwrap_or_else(|_| "0.0.0.0:8000".parse().unwrap());

    info!("Starting server on {}", addr);

    let app = Route::new()
        .nest(
            "/",
            Route::new()
                .at("", get(index_handler))
                .at("", post(parse_handler))
        )
        .at("/index.html", index_handler)
        .at("/*", catch_all)
        .with(Tracing);

    Server::new(TcpListener::bind(addr))
        .run(app)
        .await
}
