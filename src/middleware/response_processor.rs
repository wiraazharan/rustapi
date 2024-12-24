use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, body::MessageBody,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

// Middleware Struct
pub struct ResponseProcessor;

// Middleware Factory
impl<S, B> Transform<S, ServiceRequest> for ResponseProcessor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ResponseProcessorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ResponseProcessorMiddleware {
            service: Rc::new(service),
        })
    }
}

// Middleware Implementation
pub struct ResponseProcessorMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ResponseProcessorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let mut res = svc.call(req).await?;

            // Add a custom header to the response
            res.headers_mut().insert(
                actix_web::http::header::HeaderName::from_static("x-custom-header"),
                actix_web::http::header::HeaderValue::from_static("Processed by Middleware"),
            );

            Ok(res)
        })
    }
}
