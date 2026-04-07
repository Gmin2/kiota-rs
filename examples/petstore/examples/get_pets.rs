use std::sync::Arc;

use kiota_abstractions::RequestAdapter;
use kiota_abstractions::authentication::AnonymousAuthenticationProvider;
use kiota_abstractions::serialization::registry::PARSE_NODE_FACTORY_REGISTRY;
use kiota_http::HttpClientRequestAdapter;
use kiota_serialization_json::JsonParseNodeFactory;
use petstore_client::petstore_client::PetstoreClient;

#[tokio::main]
async fn main() {
    PARSE_NODE_FACTORY_REGISTRY.register(Arc::new(JsonParseNodeFactory));

    let auth = Arc::new(AnonymousAuthenticationProvider);
    let mut adapter = HttpClientRequestAdapter::new(auth).expect("failed to create adapter");
    adapter.set_base_url("https://petstore.swagger.io/v2");

    let client = PetstoreClient::new(Arc::new(adapter));

    // GET /pet/findByStatus?status=available
    println!("Fetching available pets...");
    match client.pet().find_by_status().get(None).await {
        Ok(pets) => {
            println!("Got {} pets:", pets.len());
            for pet in pets.iter().take(5) {
                println!(
                    "  [{}] {} (status: {})",
                    pet.id.unwrap_or(0),
                    pet.name.as_deref().unwrap_or("(unnamed)"),
                    pet.status.as_ref().map(|s| s.to_string()).unwrap_or_default(),
                );
            }
            if pets.len() > 5 {
                println!("  ... and {} more", pets.len() - 5);
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
