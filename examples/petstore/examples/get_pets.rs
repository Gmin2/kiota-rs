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

    // GET /pet/{petId}
    for pet_id in [1i64, 2, 3] {
        print!("GET /pet/{pet_id} -> ");
        match client.pet().by_pet_id(pet_id).get(None).await {
            Ok(Some(pet)) => {
                println!(
                    "id={} name={:?} status={:?}",
                    pet.id.unwrap_or(0),
                    pet.name.as_deref().unwrap_or("?"),
                    pet.status.as_ref().map(|s| s.to_string()).unwrap_or_default(),
                );
            }
            Ok(None) => println!("(not found)"),
            Err(e) => println!("error: {e}"),
        }
    }

    // GET /pet/findByStatus
    println!("\nGET /pet/findByStatus (available) ->");
    match client.pet().find_by_status().get(None).await {
        Ok(pets) => {
            println!("  {} pets returned", pets.len());
            for pet in pets.iter().take(3) {
                println!(
                    "  [{}] {:?}",
                    pet.id.unwrap_or(0),
                    pet.name.as_deref().unwrap_or("?"),
                );
            }
        }
        Err(e) => println!("  error: {e}"),
    }
}
