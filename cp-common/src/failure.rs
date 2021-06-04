use actix_web::HttpResponse;
use futures::future::{ok, Ready};
use futures::Future;
use std::env;
use std::pin::Pin;

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use rand::Rng;

#[derive(Default)]
pub struct FailureInjector;

impl<S> Transform<S> for FailureInjector
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = FailureInjectorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let failure_rate_env = env::var("FAILURE_RATE").unwrap_or("0".to_string());
        let failure_rate: i32 = failure_rate_env.parse().unwrap();

        ok(FailureInjectorMiddleware {
            service,
            failure_rate,
        })
    }
}
pub struct FailureInjectorMiddleware<S> {
    service: S,
    failure_rate: i32,
}

impl<S> Service for FailureInjectorMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let fut = self.service.call(req);

        let mut rng = rand::thread_rng();
        let failure = rng.gen_range(0..100) < self.failure_rate;

        Box::pin(async move {
            let mut service_res = fut.await?;

            if failure {
                *service_res.response_mut() = HttpResponse::ServiceUnavailable().finish();
            }
            Ok(service_res)
        })
    }
}
