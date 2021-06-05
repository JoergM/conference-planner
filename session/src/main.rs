use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use cp_common::delay::DelayInjector;
use cp_common::failure::FailureInjector;
use cp_common::tracing::init_jaeger_endpoint;
use env_logger::Env;
use serde::Serialize;
use serde_json::{Map, Value};
use std::{time::Duration};

mod session;
use session::*;

use actix_web_opentelemetry::{ClientExt, RequestTracing};
use opentelemetry::{Context};

#[derive(Debug, Clone)]
struct AppState {
    sessions: Vec<Session>,
}

//todo move to common
async fn get_body_with_tracing(url: &str) -> String {
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
    body_text
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
    init_jaeger_endpoint();

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
    .bind("127.0.0.1:8082")?
    .run()
    .await
}
