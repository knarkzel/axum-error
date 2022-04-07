# axum-error

```bash
cargo add axum-error
```

Provides an `Error` type which can be used in a `eyre`-like fashion for `axum`, as well as simple error handling with `fehler`.

```rust
use axum::{response::Html, routing::get, Router};
use std::{fs::read_to_string, net::SocketAddr};
use axum_error::*;
 
#[throws]
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await?;
}
 
#[throws]
async fn index() -> Html<String> {
    Html(read_to_string("index.html")?)
}
```
