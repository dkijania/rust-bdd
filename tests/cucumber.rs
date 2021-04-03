mod universe;
use cucumber_rust::{Context, Cucumber};
use rust_bdd::{api::utils::ApiContext, cucumber::debug::ProblemDetectingEventHandler};

#[tokio::test]
async fn test() {
    let api_context = ApiContext::from_env_variables();
    let debug = api_context.is_debug_enabled();

    let event_handler = ProblemDetectingEventHandler::default();
    Cucumber::<universe::TimeWorld>::with_handler(event_handler.clone())
        .features(&["./features/public/time"])
        .context(Context::new().add(api_context.clone()))
        .steps(universe::time_steps())
        .debug(debug)
        .run()
        .await;

    Cucumber::<universe::TickerWorld>::with_handler(event_handler.clone())
        .features(&["./features/public/ticker"])
        .context(Context::new().add(api_context.clone()))
        .steps(universe::ticker_steps())
        .debug(debug)
        .run()
        .await;

    Cucumber::<universe::OpenOrdersWorld>::with_handler(event_handler.clone())
        .features(&["./features/private/open_orders"])
        .context(Context::new().add(api_context))
        .steps(universe::open_orders_steps())
        .debug(debug)
        .run_and_exit()
        .await
}
