use super::OpenOrdersWorld;
use cucumber_rust::Steps;
use rust_bdd::api::utils::ApiContext;
pub fn steps() -> Steps<OpenOrdersWorld> {
    let mut steps: Steps<OpenOrdersWorld> = Steps::new();

    steps.given("User with correct api keys", |world, ctx| match world {
        OpenOrdersWorld::Define => {
            let api_context = ctx.get::<ApiContext>().expect("cannot find Api Context");
            let api_keys = api_context.keys();
            OpenOrdersWorld::BeforeRequest(api_keys.clone())
        }
        _ => panic!("Invalid world state"),
    });

    steps.given("User without api keys", |world, _ctx| match world {
        OpenOrdersWorld::Define => OpenOrdersWorld::BeforeRequest(None),
        _ => panic!("Invalid world state"),
    });

    steps.given("without any open trades", |world, _ctx| world);

    steps.when("query for open trades", |world, ctx| match world {
        OpenOrdersWorld::BeforeRequest(keys) => {
            let api = ctx.get::<ApiContext>().expect("cannot find Api Context");

            OpenOrdersWorld::AfterRequest(api.api().rest().v0().private(keys).open_orders())
        }
        _ => panic!("Invalid world state"),
    });

    steps.then("list is empty", |world, _ctx| {
        match world {
            OpenOrdersWorld::AfterRequest(ref response) => {
                let open_orders = response.as_ref().expect("unexpected failure");
                assert!(open_orders.open.is_empty())
            }
            _ => panic!("Invalid world state"),
        };
        world
    });
    steps
}
