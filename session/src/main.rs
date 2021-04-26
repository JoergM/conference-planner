use actix_service::Service;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use rand::Rng;
use serde::Serialize;
use serde_json::{Map, Value};
use std::{env, time::Duration};
use std::{thread, time};

mod session;
use session::*;

use actix_web_opentelemetry::{ClientExt, RequestTracing};
use opentelemetry::Context;

#[derive(Debug, Clone)]
struct AppState {
    sessions: Vec<Session>,
}

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
    //reading Content from environment
    let failure_rate_env = env::var("FAILURE_RATE").unwrap_or("0".to_string());
    let failure_rate: i32 = failure_rate_env.parse().unwrap();
    let random_delay_env = env::var("RANDOM_DELAY_MAX").unwrap_or("1".to_string());
    let random_delay_max: u64 = random_delay_env.parse().unwrap();

    //initialize App_State
    let app_state = AppState {
        sessions: session::generate_examples(),
    };

    // register opentelemetry collector
    let collector_env =
        env::var("OTEL_EXPORTER_JAEGER_ENDPOINT").unwrap_or("localhost:14268".to_string());
    let (_tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("Session")
        .with_collector_endpoint(format!("http://{}/api/traces", collector_env))
        .install()
        .unwrap();

    //Initialize Logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap_fn(move |req, srv| {
                let fut = srv.call(req);
                let mut rng = rand::thread_rng();
                let failure = rng.gen_range(0..100) < failure_rate;
                async move {
                    let mut service_res = fut.await?;

                    if failure {
                        *service_res.response_mut() = HttpResponse::ServiceUnavailable().finish();
                    }
                    Ok(service_res)
                }
            })
            .wrap_fn(move |req, srv| {
                let fut = srv.call(req);
                let mut rng = rand::thread_rng();
                let delay = time::Duration::from_millis(rng.gen_range(0..random_delay_max));
                async move {
                    let service_res = fut.await?;
                    thread::sleep(delay);
                    Ok(service_res)
                }
            })
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
