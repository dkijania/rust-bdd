use cucumber_rust::{event::*, output::BasicOutput, EventHandler};
use std::sync::{Arc, Mutex};

/// structs which helps debug cucumber problems
/// by allowing all println!() macros to be redirected to stdout
#[derive(Clone, Default)]
pub struct ProblemDetectingEventHandler {
    pub state: Arc<Mutex<ProblemDetectingEventHandlerState>>,
}

#[derive(Default)]
pub struct ProblemDetectingEventHandlerState {
    pub basic_output: BasicOutput,
    pub any_problem: bool,
}

impl EventHandler for ProblemDetectingEventHandler {
    
    fn handle_event(&mut self, event: &CucumberEvent) {
        let mut state = self.state.lock().unwrap();
        match &event {
            CucumberEvent::Feature(
                _,
                FeatureEvent::Scenario(
                    _,
                    ScenarioEvent::Step(_, StepEvent::Failed(StepFailureKind::Panic(_, _))),
                ),
            )
            | CucumberEvent::Feature(
                _,
                FeatureEvent::Scenario(
                    _,
                    ScenarioEvent::Step(_, StepEvent::Failed(StepFailureKind::TimedOut)),
                ),
            )
            | CucumberEvent::Feature(
                _,
                FeatureEvent::Scenario(_, ScenarioEvent::Step(_, StepEvent::Skipped)),
            )
            | CucumberEvent::Feature(
                _,
                FeatureEvent::Scenario(_, ScenarioEvent::Step(_, StepEvent::Unimplemented)),
            ) => {
                state.any_problem = true;
            }
            _ => {}
        }
        state.basic_output.handle_event(event);
    }
}
