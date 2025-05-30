use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use chrono::Utc;
use crate::models::response_model::ResponseMetadata;

pub struct ProcessingTimeMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ProcessingTimeMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ProcessingTimeMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ProcessingTimeMiddlewareService { service }))
    }
}

pub struct ProcessingTimeMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ProcessingTimeMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Utc::now().timestamp_millis();
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            
            // レスポンスボディを取得して処理時間を設定
            if let Some(metadata) = res.extensions_mut().get_mut::<ResponseMetadata>() {
                metadata.set_processing_end(Utc::now().timestamp_millis());
            }
            
            Ok(res)
        })
    }
} 