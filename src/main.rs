
#[allow(non_snake_case)]
extern crate plotters;
extern crate rust_optimal_transport as rot;

pub mod plot;
pub mod distributions;
pub mod tracking_scenario;
pub mod logger;
pub mod assignments;
pub mod ecs;

// MADS
use mads::simulator::configuration::{EngineConfig, SimulatorConfig};
use mads::simulator::simulator::Simulator;
use mads::simulator::state::SimulatorState;
use mads::ecs::resources::*;
use mads::log::logger::Logger;

// formflight
use crate::tracking_scenario::TrackingScenario;
use crate::logger::FormFlightLogger;

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

    // NOTE: post processing
    // TODO: safely unwrap resources.get()
    let time_history = simulator.state.resources.get::<SimulationTimeHistory>().unwrap();
    let result = simulator.state.resources.get::<SimulationResult>().unwrap();

    let logger = FormFlightLogger;
    if let Err(err) = logger.to_csv(&simulator.state, "./results.csv") {
        println!("csv write error, {}", err);
    };

    if let Err(err) = logger.assignments_to_json(&simulator.state, "./assignments.json") {
        println!("json write error, {}", err);
    };

    if let Err(err) = logger.sim_id_to_json(&simulator.state, "./entities.json") {
        println!("json write error, {}", err);
    };

    // (optional)
    match plot::plot_trajectory_3d(&time_history, &result) {

        Ok(()) => println!("plot done"),
        Err(_) => println!("plot error")

    };

 }

