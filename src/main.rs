use log::*;
use serde_json::json;
use trillium::Conn;
use trillium::Status;
use trillium_router::Router;
use trillium_static_compiled::include_dir;
use trillium_static_compiled::StaticCompiledHandler;
use trillium_tera::TeraConnExt;
use trillium_tera::{Tera, TeraHandler};
use url::Url;

fn render(conn: Conn) -> Conn {
    conn.assign("mf2rust_version", "0.1.0")
        .render("index.html.tera")
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Query {
    url: Url,
    html: Option<String>,
}

async fn web() {
    trace!("Setting up Microformats parsing demo website for Rust.");
    let tera = Tera::new("templates/**/*.html.tera").unwrap();

    let server = trillium_async_std::config()
        .with_nodelay()
        .with_port(8000)
        .with_host("0.0.0.0");

    server
        .run_async((
            TeraHandler::new(tera),
            trillium_logger::Logger::new(),
            trillium_conn_id::ConnId::new(),
            Router::new().any(&["get", "post"], "/*", |mut conn: Conn| async {
                if let Some(true) = conn
                    .headers()
                    .get_str("accept")
                    .map(|v| v.contains("text/html") || v.contains("*/*"))
                {
                    match serde_qs::from_str(conn.request_body_string().await.unwrap_or_default().as_str()).or_else(|_| serde_qs::from_str::<Query>(conn.querystring())) {
                        Ok(query) => {
                            let resp = match query.html {
                                None => {
                                    // FIXME: This is _not_ how one should handle this.
                                    let body = ureq::get(query.url.as_str())
                                        .call()
                                        .map(|r| r.into_string().ok())
                                        .unwrap_or_default()
                                        .unwrap_or_default();
                                    dbg!(&body);
                                    mf2::from_html(&body, query.url)
                                }
                                Some(html) => mf2::from_html(&html, query.url),
                            };

                            // FIXME: not ideal.
                            let doc = resp.unwrap_or_default();
                            let mut json = serde_json::to_value(&doc).unwrap_or_default().as_object().map(|o| o.to_owned());
                            json = json.as_mut().map(|o| {
                                o.insert("debug".to_owned(), json!({
 "package": "https://crates.io/crates/microformats2",
    "version": "0.1.0",
    "note": [
      "This output was generated from the microformats2 crate available at https://gitlab.com/maxburon/microformats-parser.",
      "Please file any issues with the parser at https://gitlab.com/maxburon/microformats-parser/issues"
    ]
                                })); o.to_owned()
                            });

                            conn.with_body(serde_json::to_string_pretty(&json).unwrap_or_default())
                                .with_header("content-type", "application/json; utf-8")
                                .with_status(Status::Ok)
                                .halt()
                        }
                        Err(e) => {
                            trace!("Failed to parse incoming request: {:#?}", e);
                            render(conn)
                        }
                    }
                } else {
                    conn
                }
            }),
            StaticCompiledHandler::new(include_dir!("static")).with_index_file("index.html"),
        ))
        .await
}

#[async_std::main]
async fn main() {
    env_logger::init();
    web().await
}
