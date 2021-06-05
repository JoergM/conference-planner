use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use cp_common::delay::DelayInjector;
use cp_common::failure::FailureInjector;
use cp_common::tracing::*;
use env_logger::Env;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use serde::Serialize;
use serde_json::{Map, Value};

mod schedule_entries;
use schedule_entries::*;

#[derive(Debug, Clone)]
struct AppState {
    schedule_entries: Vec<ScheduleEntry>,
}

#[derive(Serialize)]
struct ScheduleAnswer {
    id: u32,
    start_time: String,
    end_time: String,
    session_id: u32,
    session_title: String,
    session_tag: String,
    speaker_name: String,
}

impl ScheduleAnswer {
    fn new(entry: ScheduleEntry, session_data: Map<String, Value>) -> Self {
        let session_title = session_data["title"].as_str().unwrap_or("");
        let session_tag = session_data["tag"].as_str().unwrap_or("");
        let speaker_name = session_data["speaker_name"].as_str().unwrap_or("");

        ScheduleAnswer {
            id: entry.id,
            start_time: entry.start_time,
            end_time: entry.end_time,
            session_id: entry.session_id,
            session_title: session_title.into(),
            session_tag: session_tag.into(),
            speaker_name: speaker_name.into(),
        }
    }
}

async fn get_session_value(id: u32) -> Map<String, Value> {
    let url = format!("http://sessions:8082/{}", id);
    let body_text = get_body_with_tracing(&url).await;
    let data: Map<String, Value> = serde_json::from_str(&body_text).unwrap();

    data
}

#[get("/")]
async fn list(scope: web::Data<AppState>) -> impl Responder {
    let mut answers: Vec<ScheduleAnswer> = Vec::new();

    for entry in scope.schedule_entries.iter() {
        let session_data = get_session_value(entry.session_id).await;
        let schedule_answer = ScheduleAnswer::new(entry.clone(), session_data);
        answers.push(schedule_answer)
    }

    let json = serde_json::to_string(&answers).unwrap();

    HttpResponse::Ok().body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //initialize App_State
    let app_state = AppState {
        schedule_entries: schedule_entries::generate_examples(),
    };

    // register opentelemetry collector
    global::set_text_map_propagator(TraceContextPropagator::new());
    let (_tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("Schedule")
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
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
