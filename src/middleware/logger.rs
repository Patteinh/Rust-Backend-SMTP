use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{self, Ready};
use std::sync::Mutex;
use std::task::{Context, Poll};

pub struct Logger;

impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(LoggerMiddleware {
            service: Mutex::new(service),
        }))
    }
}

pub struct LoggerMiddleware<S> {
    service: Mutex<S>,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let service = self.service.lock().expect("Mutex is poisoned");
        service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Received request: {} {}", req.method(), req.uri());
        let service = self.service.lock().expect("Mutex is poisoned");
        service.call(req)
    }
}
