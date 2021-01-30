use serde::Deserialize;
use serde::Serialize;
use std::{borrow::Cow, fmt::Display};

use actix_web::{HttpResponse, Responder, ResponseError, http::{StatusCode}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResult<T = ()> {
    pub code: u16,
    pub msg: Option<Cow<'static, str>>,
    pub data: Option<T>,
}

// impl <T>Display for ApiResult<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         format!("{:?}", self)
//     }
// }

impl<T: Serialize> ApiResult<T> {
    pub fn new() -> Self {
        Self {
            code: 200,
            msg: None,
            data: None,
        }
    }
    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }
    pub fn with_msg<S: Into<Cow<'static, str>>>(mut self, msg: S) -> Self {
        self.msg = Some(msg.into());
        self
    }
    pub fn msg_as_str(&self) -> &str {
        self.msg.as_ref().map(|s| s.as_ref()).unwrap_or_default()
    }
    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
    pub fn to_resp(&self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .status(StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .json(self)
    }
}

// impl <T>futures::Future for ApiResult<T> {
//     type Output;

//     fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         todo!()
//     }
// }

// impl <T> ResponseError for ApiResult<T> where T: std::fmt::Debug {
//     fn status_code(&self) -> StatusCode {
//         StatusCode::INTERNAL_SERVER_ERROR
//     }

//     fn error_response(&self) -> HttpResponse {
//         let mut resp = HttpResponse::new(self.status_code());
//         let mut buf = actix_web::web::BytesMut::new();
//         resp.headers_mut().insert(
//             actix_web::http::header::CONTENT_TYPE,
//             actix_web::http::HeaderValue::from_static("text/plain; charset=utf-8"),
//         );
//         resp.set_body(actix_web::dev::Body::from(buf))
//     }

//     // fn__private_get_type_id__(&self)->(std::any::TypeId,actix_web::error::PrivateHelper)whereSelf: 'static,{(std::any::TypeId::of::<Self>(),actix_web::error::PrivateHelper(()))}
// }


// impl <T> Responder for ApiResult<T> {
//     type Error = ApiResult<()>;

//     type Future  = ApiResult<T>;

//     fn respond_to(self, req: &actix_web::HttpRequest) -> Self::Future {
//         let res = HttpResponse::Ok()
//             .content_type("application/json")
//             .status(StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
//             .json(self);
//         res
//     }
// }
