use actix_service::Service;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use rand::Rng;
use std::env;
use std::{thread, time};

mod speaker;
use speaker::*;

#[derive(Debug, Clone)]
struct AppState {
    speakers: Vec<Speaker>,
}

#[get("/")]
async fn list(scope: web::Data<AppState>) -> impl Responder {
    let json = serde_json::to_string(&scope.speakers).unwrap();

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
        speakers: speaker::generate_examples(),
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
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
