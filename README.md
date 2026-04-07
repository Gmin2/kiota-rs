# kiota-rust

Rust language support for [Microsoft Kiota](https://github.com/microsoft/kiota) an OpenAPI SDK generator.

Generate strongly-typed Rust API clients from any OpenAPI description.

## Quick start

Generate a client:

```sh
kiota generate -l Rust -c PostsClient -n posts_client \
  -d https://raw.githubusercontent.com/microsoft/kiota-samples/main/get-started/quickstart/posts-api.yml \
  -o ./src
```

Use it:

```rust
use std::sync::Arc;
use kiota_abstractions::RequestAdapter;
use kiota_abstractions::authentication::AnonymousAuthenticationProvider;
use kiota_abstractions::serialization::registry::PARSE_NODE_FACTORY_REGISTRY;
use kiota_http::HttpClientRequestAdapter;
use kiota_serialization_json::JsonParseNodeFactory;

#[tokio::main]
async fn main() {
    PARSE_NODE_FACTORY_REGISTRY.register(Arc::new(JsonParseNodeFactory));

    let auth = Arc::new(AnonymousAuthenticationProvider);
    let mut adapter = HttpClientRequestAdapter::new(auth).unwrap();
    adapter.set_base_url("https://jsonplaceholder.typicode.com");

    let client = PostsClient::new(Arc::new(adapter));
    let posts = client.posts().get(None).await.unwrap();
    println!("Got {} posts", posts.len());
}
```

## Crates

| Crate | Description |
|-------|-------------|
| `kiota-abstractions` | Core traits: `Parsable`, `ParseNode`, `SerializationWriter`, `RequestAdapter` |
| `kiota-http` | `reqwest`-based HTTP adapter with middleware pipeline |
| `kiota-serialization-json` | JSON serialization via `serde_json` |
| `kiota-serialization-form` | URL-encoded form serialization |
| `kiota-serialization-text` | Plain text serialization |
| `kiota-serialization-multipart` | Multipart form-data serialization |
| `kiota-bundle` | Convenience re-exports with default factory registration |
| `kiota-authentication` | GNAP auth provider for Open Payments (stub) |

## Examples

```sh
# Fetch posts from JSONPlaceholder
cargo run --example get_posts -p posts-client

# Fetch pets from Swagger Petstore
cargo run --example get_pets -p petstore-client

# Fetch latest releases from GitHub
cargo run --example get_release -p github-releases-client
```

## Dependencies

Generated crates need these in `Cargo.toml`:

```toml
[dependencies]
kiota-abstractions = { path = "..." }
kiota-http = { path = "..." }
kiota-serialization-json = { path = "..." }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
# add if your API uses these types:
chrono = { version = "0.4", default-features = false, features = ["std"] }
uuid = "1"
```

Or use the bundle:

```toml
[dependencies]
kiota-bundle = { path = "..." }
tokio = { version = "1", features = ["full"] }
```

## License

MIT
