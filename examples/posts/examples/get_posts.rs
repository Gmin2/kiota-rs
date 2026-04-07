use std::sync::Arc;

use kiota_abstractions::RequestAdapter;
use kiota_abstractions::authentication::AnonymousAuthenticationProvider;
use kiota_abstractions::serialization::registry::PARSE_NODE_FACTORY_REGISTRY;
use kiota_http::HttpClientRequestAdapter;
use kiota_serialization_json::JsonParseNodeFactory;
use posts_client::posts_client::PostsClient;

#[tokio::main]
async fn main() {
    // register JSON deserializer
    PARSE_NODE_FACTORY_REGISTRY.register(Arc::new(JsonParseNodeFactory));

    // create the adapter with anonymous auth (jsonplaceholder doesn't need auth)
    let auth = Arc::new(AnonymousAuthenticationProvider);
    let mut adapter = HttpClientRequestAdapter::new(auth).expect("failed to create adapter");
    adapter.set_base_url("https://jsonplaceholder.typicode.com");

    // create the client
    let client = PostsClient::new(Arc::new(adapter));

    // GET /posts
    println!("Fetching posts...");
    match client.posts().get(None).await {
        Ok(posts) => {
            println!("Got {} posts:", posts.len());
            for post in posts.iter().take(3) {
                println!(
                    "  [{}] {} - {}",
                    post.id.unwrap_or(0),
                    post.title.as_deref().unwrap_or("(no title)"),
                    post.body.as_deref().unwrap_or("").chars().take(50).collect::<String>(),
                );
            }
            if posts.len() > 3 {
                println!("  ... and {} more", posts.len() - 3);
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
