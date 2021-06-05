use actix_web_opentelemetry::ClientExt;
use opentelemetry::Context;
use std::{env, time::Duration};

pub fn get_collector_endpoint() -> String {
    let collector_env =
        env::var("OTEL_EXPORTER_JAEGER_ENDPOINT").unwrap_or("localhost:14268".to_string());
    format!("http://{}/api/traces", collector_env)
}

pub async fn get_body_with_tracing(url: &str) -> String {
    let client = awc::Client::default();
    let mut resp = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .trace_request_with_context(Context::current())
        .send()
        .await
        .unwrap();
    let body = resp.body().await.unwrap();
    let body_text = String::from_utf8(body.to_vec()).unwrap();
    dbg!(&body_text);
    body_text
}
