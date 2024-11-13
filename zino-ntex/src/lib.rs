#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://zino.cc/assets/zino-logo.png")]
#![doc(html_logo_url = "https://zino.cc/assets/zino-logo.svg")]
#![allow(async_fn_in_trait)]
#![forbid(unsafe_code)]

mod application;
mod request;
mod response;

pub use application::Cluster;
pub use request::Extractor;
pub use response::{NtexRejection, NtexResponse};

/// Router configure.
pub type RouterConfigure = fn(cfg: &mut ntex::web::ServiceConfig);

/// A specialized request extractor.
pub type Request = Extractor<ntex::web::HttpRequest>;

/// A specialized response.
pub type Response = zino_core::response::Response<ntex::http::StatusCode>;

/// A specialized `Result` type.
pub type Result<T = NtexResponse> = std::result::Result<T, NtexRejection>;
