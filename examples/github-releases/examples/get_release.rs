use std::sync::Arc;

use kiota_abstractions::RequestAdapter;
use kiota_abstractions::authentication::AnonymousAuthenticationProvider;
use kiota_abstractions::serialization::registry::PARSE_NODE_FACTORY_REGISTRY;
use kiota_http::HttpClientRequestAdapter;
use kiota_serialization_json::JsonParseNodeFactory;
use github_releases_client::git_hub_releases_client::GitHubReleasesClient;

#[tokio::main]
async fn main() {
    PARSE_NODE_FACTORY_REGISTRY.register(Arc::new(JsonParseNodeFactory));

    let auth = Arc::new(AnonymousAuthenticationProvider);
    let mut adapter = HttpClientRequestAdapter::new(auth).expect("failed to create adapter");
    adapter.set_base_url("https://api.github.com");

    let client = GitHubReleasesClient::new(Arc::new(adapter));

    let repos = [
        ("microsoft", "kiota"),
        ("anthropics", "claude-code"),
        ("tokio-rs", "tokio"),
    ];

    for (owner, repo) in repos {
        print!("{owner}/{repo} latest -> ");
        match client
            .repos()
            .by_owner(owner.to_string())
            .by_repo(repo.to_string())
            .releases()
            .by_release_name("latest".to_string())
            .get(None)
            .await
        {
            Ok(Some(release)) => {
                println!(
                    "{} ({}) - published {}",
                    release.tag_name.as_deref().unwrap_or("?"),
                    release.name.as_deref().unwrap_or("?"),
                    release.published_at.map(|d| d.to_string()).unwrap_or_default(),
                );
            }
            Ok(None) => println!("(no release)"),
            Err(e) => println!("error: {e}"),
        }
    }
}
