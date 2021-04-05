use super::TickerWorld;
use cucumber_rust::Steps;
use rust_bdd::api::utils::{get_value, ApiContext};

pub fn steps() -> Steps<TickerWorld> {
    let mut steps: Steps<TickerWorld> = Steps::new();

    steps.given("User without api keys", |_world, _ctx| {
        TickerWorld::BeforeRequest
    });

    steps.when_regex(
        r#"^query for ticker pair "(.*)"$"#,
        |world, ctx| match world {
            TickerWorld::BeforeRequest => {
                let api_context = ctx.get::<ApiContext>().expect("cannot find Api Context");
                let api = api_context.api();
                let pair = ctx
                    .matches
                    .get(1)
                    .expect("cannot extract pari from sentence");
                TickerWorld::AfterRequest(api.rest().v0().public().ticker(pair))
            }
            _ => panic!("Invalid world state"),
        },
    );

    steps.then_regex(
        r#"^field "(.*)" in response contains "(.*)"$"#,
        |world, ctx| {
            match world {
                TickerWorld::AfterRequest(ref response) => {
                    let tickers = response.as_ref().expect("unexpected response failure");
                    assert_eq!(tickers.len(), 1, "only one pair was expected");
                    let ticker = tickers.values().next().unwrap();
                    let name = ctx.matches.get(1).expect("cannot get name from sentence");
                    let expected_part = ctx.matches.get(2).expect("cannot get value from sentence");
                    let actual_value = get_value(ticker, name);
                    assert!(
                        actual_value.contains(expected_part),
                        "{} of response does not contains value '{}' in '{}'",
                        name,
                        actual_value,
                        expected_part
                    );
                }
                _ => panic!("Invalid world state"),
            };
            world
        },
    );

    steps.then_regex(
        r#"^error is produced with messsage ~ "(.*)"$"#,
        |world, ctx| {
            match world {
                TickerWorld::AfterRequest(ref response) => {
                    let error = response
                        .as_ref()
                        .err()
                        .expect("expected response to contain an error");
                    let expected_error_msg_part = ctx
                        .matches
                        .get(1)
                        .expect("cannot get error message from sentence");
                    let error_message = error.to_string();
                    assert!(
                        error_message.contains(expected_error_msg_part),
                        "error message does not contains '{}' (full message: '{}')",
                        expected_error_msg_part,
                        error_message
                    );
                }
                _ => panic!("Invalid world state"),
            };
            world
        },
    );
    steps
}
