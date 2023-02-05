//! Provides an `Error` type which can be used in a `eyre`-like fashion
//! for `axum`
//!
//! ```rust
//! use axum::{response::Html, routing::get, Router};
//! use std::{fs::read_to_string, net::SocketAddr};
//! use axum_error::Result;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new().route("/", get(index));
//!     axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap()
//! }
//! 
//! async fn index() -> Result<Html<String>> {
//!     Ok(Html(read_to_string("index.html")?))
//! }
//! ```

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Error type which implements `IntoResponse`
#[derive(Debug)]
pub struct Error(pub eyre::Report);

impl<E: Into<eyre::Report>> From<E> for Error {
    fn from(error: E) -> Self {
        Error(error.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.0)).into_response()
    }
}

// Result type
pub type Result<T, E = Error> = std::result::Result<T, E>;
