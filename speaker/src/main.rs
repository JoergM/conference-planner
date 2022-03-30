use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};

use cp_common::delay::DelayInjector;
use cp_common::failure::FailureInjector;
use cp_common::tracing::*;

mod speaker;
use speaker::*;

use actix_web_opentelemetry::RequestTracing;

#[derive(Debug, Clone)]
struct AppState {
    speakers: Vec<Speaker>,
}

#[get("/")]
async fn list(scope: web::Data<AppState>) -> impl Responder {
    let json = serde_json::to_string(&scope.speakers).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/{id}")]
async fn speaker_by_id(
    web::Path(id): web::Path<u32>,
    scope: web::Data<AppState>,
) -> impl Responder {
    let speaker = scope.speakers.iter().find(|speaker| speaker.id == id);

    if speaker.is_some() {
        let json = serde_json::to_string(&speaker).unwrap();
        HttpResponse::Ok().body(json)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //initialize App_State
    let app_state = AppState {
        speakers: speaker::generate_examples(),
    };

    // register opentelemetry collector
    global::set_text_map_propagator(TraceContextPropagator::new());
    let (_tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("Speakers")
        .with_collector_endpoint(get_collector_endpoint())
        .install()
        .unwrap();

    //Initialize Logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(DelayInjector::default())
            .wrap(FailureInjector::default())
            .wrap(RequestTracing::new())
            .wrap(Logger::default())
            .data(app_state.clone())
            .service(list)
            .service(speaker_by_id)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
