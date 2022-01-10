
#[allow(non_snake_case)]
extern crate plotters;
extern crate rust_optimal_transport as ot;

pub mod plot;
pub mod distributions;
pub mod tracking_scenario;
pub mod logger;
pub mod assignments;
pub mod ecs;
pub mod post_process;

// MADS
use mads::simulator::configuration::{EngineConfig, SimulatorConfig};
use mads::simulator::Simulator;
use mads::simulator::state::SimulatorState;

// formflight
use crate::tracking_scenario::TrackingScenario;
use crate::post_process::*;

fn main() {

    // Configure MADS simulator
    let engine_config = EngineConfig::default();
    let sim_config = SimulatorConfig::default();
    let sim_state = SimulatorState::new(engine_config, sim_config);

    // Configure Scenario
    let scenario = TrackingScenario::default();

    // Simulate
    let mut simulator = Simulator::new(sim_state, scenario);
    simulator.build();
    simulator.run();

    // Post-Process
    post_process(&simulator);

 }

