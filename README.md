# axum-eyre

```bash
cargo add axum-eyre
```

Crate which provides an `Error` and `Result` type which
can be used in a `eyre`-like fashion.

```rust
use axum::{response::Html, routing::get, Router};
use std::{fs::read_to_string, net::SocketAddr};
use axum_eyre::Result;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Result<Html<String>> {
    let template = read_to_string("index.html")?;
    Ok(Html(template))
}
```
