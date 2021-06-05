use core::time;
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::{env, thread};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

use rand::Rng;

#[derive(Default)]
pub struct DelayInjector;

impl<S> Transform<S> for DelayInjector
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = DelayInjectorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let random_delay_env = env::var("RANDOM_DELAY_MAX").unwrap_or("1".to_string());
        let random_delay_max: u64 = random_delay_env.parse().unwrap();

        ok(DelayInjectorMiddleware {
            service,
            random_delay_max,
        })
    }
}
pub struct DelayInjectorMiddleware<S> {
    service: S,
    random_delay_max: u64,
}

impl<S> Service for DelayInjectorMiddleware<S>
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
        let delay = time::Duration::from_millis(rng.gen_range(0..self.random_delay_max));

        Box::pin(async move {
            let service_res = fut.await?;
            thread::sleep(delay);
            Ok(service_res)
        })
    }
}
