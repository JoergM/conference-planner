use actix_service::Service;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use rand::Rng;
use serde::Serialize;
use serde_json::{Map, Value};
use std::env;
use std::{thread, time};

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

impl From<ScheduleEntry> for ScheduleAnswer {
    fn from(entry: ScheduleEntry) -> Self {
        //todo implement request to speakers
        // let data = get_speaker_value(entry.speaker_id);

        // let speaker_name = data["full_name"].as_str().unwrap_or("");
        // let speaker_twitter = data["twitter"].as_str().unwrap_or("");

        ScheduleAnswer {
            id: entry.id,
            start_time: entry.start_time,
            end_time: entry.end_time,
            session_id: entry.session_id,
            session_title: "Relativity is King".into(),
            session_tag: "Keynote".into(),
            speaker_name: "Albert Einstein".into(),
        }
    }
}

fn get_speaker_value(id: u32) -> Map<String, Value> {
    let url = format!("http://speakers:8081/{}", id);
    let resp = reqwest::blocking::get(&url).unwrap();
    let data: Map<String, Value> = serde_json::from_str(&resp.text().unwrap()).unwrap();

    data
}

#[get("/")]
async fn list(scope: web::Data<AppState>) -> impl Responder {
    let answers: Vec<ScheduleAnswer> = scope
        .schedule_entries
        .iter()
        .map(|entry| ScheduleAnswer::from(entry.clone()))
        .collect();
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
            .wrap(Logger::default())
            .data(app_state.clone())
            .service(list)
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
