use actix_service::Service;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::{ClientExt, RequestTracing};
use env_logger::Env;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator, Context};
use rand::Rng;
use serde::Serialize;
use serde_json::{Map, Value};
use std::{env, time::Duration};
use std::{thread, time};

mod schedule_entries;
use schedule_entries::*;

#[derive(Debug, Clone)]
struct AppState {
    schedule_entries: Vec<ScheduleEntry>,
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
    //reading Content from environment
    let failure_rate_env = env::var("FAILURE_RATE").unwrap_or("0".to_string());
    let failure_rate: i32 = failure_rate_env.parse().unwrap();
    let random_delay_env = env::var("RANDOM_DELAY_MAX").unwrap_or("1".to_string());
    let random_delay_max: u64 = random_delay_env.parse().unwrap();

    //initialize App_State
    let app_state = AppState {
        schedule_entries: schedule_entries::generate_examples(),
    };

    // register opentelemetry collector
    let collector_env =
        env::var("OTEL_EXPORTER_JAEGER_ENDPOINT").unwrap_or("localhost:14268".to_string());
    global::set_text_map_propagator(TraceContextPropagator::new());
    let (_tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("Schedule")
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
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
