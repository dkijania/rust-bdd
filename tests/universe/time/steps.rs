use super::TimeWorld;
use cucumber_rust::Steps;
use rust_bdd::api::utils::{self, ApiContext};

pub fn steps() -> Steps<TimeWorld> {
    let mut steps: Steps<TimeWorld> = Steps::new();

    steps.given("User without api keys", |_world, _ctx| {
        TimeWorld::BeforeRequest
    });

    steps.when("query for server time", |world, ctx| match world {
        TimeWorld::BeforeRequest => {
            let api_context = ctx.get::<ApiContext>().expect("cannot find Api Context");
            let api = api_context.api();
            TimeWorld::AfterRequest(api.rest().v0().public().server_time())
        }
        _ => panic!("Invalid world state"),
    });

    steps.then("date close to current time is produced", |world, _ctx| {
        match world {
            TimeWorld::AfterRequest(ref response) => {
                let time_from_api = response.as_ref().expect("unexpected response failure");
                let current_time = utils::current_time_unix();
                assert!(time_from_api.unixtime - current_time <= 5);
            }
            _ => panic!("Invalid world state"),
        };
        world
    });

    steps.then_regex(r#"human readable date has "(.*)" format$"#, |world, ctx| {
        match world {
            TimeWorld::AfterRequest(ref response) => {
                let time_from_api = response.as_ref().expect("unexpected response failure");
                let format = ctx
                    .matches
                    .get(1)
                    .expect("cannot extract date format from sentence");
                let unixtime_frc1123 = utils::parse_as_unixtime(&time_from_api.rfc1123, format)
                    .expect("cannot parse date");
                assert_eq!(time_from_api.unixtime, unixtime_frc1123);
            }
            _ => panic!("Invalid world state"),
        };
        world
    });
    steps
}
