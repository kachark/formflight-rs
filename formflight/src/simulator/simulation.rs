
use crate::simulator::state::{EngineState, SimulationState};
use crate::scene::scenario::Scenario;

pub struct Simulation<T>
where
    T: Scenario
{

    // TODO: don't keep public
    pub state: SimulationState,
    pub scenario: T

}

impl<T> Simulation<T>
where
    T: Scenario
{

    pub fn new(state: SimulationState, scenario: T) -> Self {

        Self { state, scenario }

    }

    /// Builds Simulation using Simulation state ECS World and Resources
    pub fn build(&mut self) {

        // Add resources to simulation state and prepare scenario internal state
        self.scenario.setup(&mut self.state.world, &mut self.state.resources);

        // Define simulation state schedule
        self.state.schedule = self.scenario.build();

    }

    /// Simulation loop
    pub fn run(&mut self) {

        self.state.status = EngineState::Active;

        while self.state.status == EngineState::Active {

            // Execute ECS systems
            self.state.update();

            // update scenario or perform additional operations / queries on entities
            self.scenario.update(&mut self.state.world, &mut self.state.resources);

        }

    }

}

