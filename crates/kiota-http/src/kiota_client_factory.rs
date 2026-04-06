use crate::middleware::Middleware;
use crate::retry_handler::RetryHandler;
use crate::user_agent_handler::UserAgentHandler;

pub fn create_default() -> (reqwest::Client, Vec<Box<dyn Middleware>>) {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none()) // let middleware handle redirects
        .timeout(std::time::Duration::from_secs(100))
        .build()
        .expect("failed to build reqwest client");

    let middlewares: Vec<Box<dyn Middleware>> = vec![
        Box::new(UserAgentHandler),
        Box::new(RetryHandler::default()),
    ];

    (client, middlewares)
}
