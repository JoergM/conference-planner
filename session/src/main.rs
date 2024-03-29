use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use cp_common::delay::DelayInjector;
use cp_common::failure::FailureInjector;
use cp_common::tracing::*;
use env_logger::Env;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use opentelemetry_jaeger::new_pipeline;
use serde::Serialize;
use serde_json::{Map, Value};

mod session;
use session::*;

#[derive(Debug, Clone)]
struct AppState {
    sessions: Vec<Session>,
}

#[derive(Serialize)]
struct SessionAnswer {
    id: u32,
    title: String,
    tag: String,
    description: String,
    speaker_id: u32,
    speaker_name: String,
    speaker_twitter: String,
}

impl SessionAnswer {
    fn new(session: &Session, speaker_data: Map<String, Value>) -> Self {
        let speaker_name = speaker_data["full_name"].as_str().unwrap_or("");
        let speaker_twitter = speaker_data["twitter"].as_str().unwrap_or("");

        SessionAnswer {
            id: session.id,
            title: session.title.clone(),
            tag: session.tag.clone(),
            description: session.description.clone(),
            speaker_id: session.speaker_id,
            speaker_name: speaker_name.into(),
            speaker_twitter: speaker_twitter.into(),
        }
    }
}

async fn get_speaker_value(id: u32) -> Map<String, Value> {
    let url = format!("http://speakers:8081/{}", id);
    let body_text = get_body_with_tracing(&url).await;
    let data: Map<String, Value> = serde_json::from_str(&body_text).unwrap();

    data
}

#[get("/")]
async fn list(scope: web::Data<AppState>) -> impl Responder {
    let mut answers: Vec<SessionAnswer> = Vec::new();

    for session in scope.sessions.iter() {
        let speaker_data = get_speaker_value(session.id).await;
        let session_answer = SessionAnswer::new(session, speaker_data);
        answers.push(session_answer)
    }
    let json = serde_json::to_string(&answers).unwrap();

    HttpResponse::Ok().body(json)
}

#[get("/{id}")]
async fn session_by_id(
    web::Path(id): web::Path<u32>,
    scope: web::Data<AppState>,
) -> impl Responder {
    let session_opt = scope.sessions.iter().find(|session| session.id == id);

    if session_opt.is_some() {
        let session = session_opt.unwrap();
        let speaker_data = get_speaker_value(session.id).await;
        let session_answer = SessionAnswer::new(session, speaker_data);
        let json = serde_json::to_string(&session_answer).unwrap();
        HttpResponse::Ok().body(json)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //initialize App_State
    let app_state = AppState {
        sessions: session::generate_examples(),
    };

    // register opentelemetry collector
    global::set_text_map_propagator(TraceContextPropagator::new());
    let (_tracer, _uninstall) = new_pipeline()
        .with_service_name("Session")
        .with_collector_endpoint(get_collector_endpoint())
        .install()
        .unwrap();

    //Initialize Logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    //todo move to common
    HttpServer::new(move || {
        App::new()
            .wrap(DelayInjector::default())
            .wrap(FailureInjector::default())
            .wrap(RequestTracing::new())
            .wrap(Logger::default())
            .data(app_state.clone())
            .service(list)
            .service(session_by_id)
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}
