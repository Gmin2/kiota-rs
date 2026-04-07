use std::sync::Arc;

use kiota_abstractions::RequestAdapter;
use kiota_abstractions::authentication::AnonymousAuthenticationProvider;
use kiota_abstractions::serialization::registry::PARSE_NODE_FACTORY_REGISTRY;
use kiota_abstractions::RequestConfiguration;
use kiota_http::HttpClientRequestAdapter;
use kiota_serialization_json::JsonParseNodeFactory;
use posts_client::posts_client::PostsClient;
use posts_client::posts::posts_request_builder::PostsRequestBuilderGetQueryParameters;

#[tokio::main]
async fn main() {
    PARSE_NODE_FACTORY_REGISTRY.register(Arc::new(JsonParseNodeFactory));

    let auth = Arc::new(AnonymousAuthenticationProvider);
    let mut adapter = HttpClientRequestAdapter::new(auth).expect("failed to create adapter");
    adapter.set_base_url("https://jsonplaceholder.typicode.com");

    let client = PostsClient::new(Arc::new(adapter));

    // GET /posts (all)
    println!("GET /posts (all):");
    match client.posts().get(None).await {
        Ok(posts) => println!("  {} posts total", posts.len()),
        Err(e) => eprintln!("  error: {e}"),
    }

    // GET /posts?userId=1 (filtered by query param)
    println!("\nGET /posts?userId=1:");
    let config = RequestConfiguration {
        query_parameters: Some(PostsRequestBuilderGetQueryParameters {
            user_id: Some("1".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    match client.posts().get(Some(&config)).await {
        Ok(posts) => {
            println!("  {} posts by user 1:", posts.len());
            for post in posts.iter().take(3) {
                println!(
                    "  [{}] {}",
                    post.id.unwrap_or(0),
                    post.title.as_deref().unwrap_or("?"),
                );
            }
            if posts.len() > 3 {
                println!("  ... and {} more", posts.len() - 3);
            }
        }
        Err(e) => eprintln!("  error: {e}"),
    }
}
