use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use opentelemetry_jaeger::new_pipeline;
use std::env;

pub fn init_jaeger_endpoint() {
    let collector_env =
        env::var("OTEL_EXPORTER_JAEGER_ENDPOINT").unwrap_or("localhost:14268".to_string());
    global::set_text_map_propagator(TraceContextPropagator::new());
    let (_tracer, _uninstall) = new_pipeline()
        .with_service_name("Session")
        .with_collector_endpoint(format!("http://{}/api/traces", collector_env))
        .install()
        .unwrap();
}
