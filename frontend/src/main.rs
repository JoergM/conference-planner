use actix_service::Service;
use actix_web::{
    get,
    middleware::Logger,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use rand::Rng;
use serde_json::json;
use std::{env, sync::Arc, time::Duration};
use std::{thread, time};

use handlebars::{Handlebars, JsonValue};

use actix_web_opentelemetry::{ClientExt, RequestTracing};
use opentelemetry::Context;

#[derive(Debug, Clone)]
struct AppState<'a> {
    hb: Arc<Handlebars<'a>>,
    alternate_design: bool,
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

#[get("/")]
async fn index(scope: web::Data<AppState<'_>>) -> impl Responder {
    let data = json!({
        "alternate_design": scope.alternate_design,
    });

    let hb = scope.hb.clone();

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/speakers/")]
async fn speakers(scope: web::Data<AppState<'_>>) -> impl Responder {
    let body_text = get_body_with_tracing("http://speakers:8081").await;

    let speakers: JsonValue = serde_json::from_str(&body_text).unwrap();
    let data = json!({
        "alternate_design": scope.alternate_design,
        "speakers": speakers,
    });

    let hb = scope.hb.clone();

    let body = hb.render("speakers", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/schedule/")]
async fn schedule(scope: web::Data<AppState<'_>>) -> impl Responder {
    let body_text = get_body_with_tracing("http://schedule:8083").await;

    let schedule: JsonValue = serde_json::from_str(&body_text).unwrap();
    let data = json!({
        "alternate_design": scope.alternate_design,
        "schedule": schedule,
    });

    let hb = scope.hb.clone();

    let body = hb.render("schedule", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/sessions/")]
async fn sessions(scope: web::Data<AppState<'_>>) -> impl Responder {
    let body_text = get_body_with_tracing("http://sessions:8082").await;
    let sessions: JsonValue = serde_json::from_str(&body_text).unwrap();
    let data = json!({
        "alternate_design": scope.alternate_design,
        "sessions": sessions,
    });

    let hb = scope.hb.clone();

    let body = hb.render("sessions", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //read failure rate
    let failure_rate_env = env::var("FAILURE_RATE").unwrap_or("0".to_string());
    let failure_rate: i32 = failure_rate_env.parse().unwrap();

    //read Random Delay
    let random_delay_env = env::var("RANDOM_DELAY_MAX").unwrap_or("1".to_string());
    let random_delay_max: u64 = random_delay_env.parse().unwrap();

    //setting alternate design
    let alternate_design_env = env::var("ALTERNATE_DESIGN");
    let alternate_design;
    if alternate_design_env.is_ok() {
        alternate_design = true;
    } else {
        alternate_design = false;
    }

    // register opentelemetry collector
    let collector_env =
        env::var("OTEL_EXPORTER_JAEGER_ENDPOINT").unwrap_or("localhost:14268".to_string());
    let (_tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("Frontend")
        .with_collector_endpoint(format!("http://{}/api/traces", collector_env))
        .install()
        .unwrap();

    //register handlebars
    let mut hb = Handlebars::new();
    hb.register_template_string("index", include_str!("templates/index.html"))
        .unwrap();
    hb.register_template_string("speakers", include_str!("templates/speakers/index.html"))
        .unwrap();
    hb.register_template_string("schedule", include_str!("templates/schedule/index.html"))
        .unwrap();
    hb.register_template_string("sessions", include_str!("templates/sessions/index.html"))
        .unwrap();

    //initialize App_State
    let app_state = AppState {
        hb: Arc::new(hb),
        alternate_design,
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
            .wrap(RequestTracing::new())
            .wrap(Logger::default())
            .data(app_state.clone())
            .service(index)
            .service(speakers)
            .service(schedule)
            .service(sessions)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
