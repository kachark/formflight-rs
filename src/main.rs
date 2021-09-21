
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
use mads::simulator::configuration::{EngineConfig, SimulationConfig};
use mads::simulator::simulation::Simulation;
use mads::simulator::state::SimulationState;
use mads::ecs::resources::*;
use mads::log::simulation_logger::SimulationLogger;

// formflight
use crate::tracking_scenario::TrackingScenario;
use crate::logger::Logger;

fn main() {

    // Configure simulation, engine, and scenario
    let engine_config = EngineConfig::default();
    let sim_config = SimulationConfig::default();
    let sim_state = SimulationState::new(engine_config, sim_config);
    let scenario = TrackingScenario::default();

    let mut simulation = Simulation::new(sim_state, scenario);
    simulation.build();
    simulation.run();

    // NOTE: post processing
    // TODO: safely unwrap resources.get()
    let time_history = simulation.state.resources.get::<SimulationTimeHistory>().unwrap();
    let result = simulation.state.resources.get::<SimulationResult>().unwrap();

    let logger = Logger;
    if let Err(err) = logger.to_csv(&simulation.state) {
        println!("csv write error, {}", err);
    };

    if let Err(err) = logger.assignments_to_json(&simulation.state) {
        println!("csv write error, {}", err);
    };

    if let Err(err) = logger.sim_id_to_json(&simulation.state) {
        println!("csv write error, {}", err);
    };

    // (optional)
    match plot::plot_trajectory_3d(&time_history, &result) {

        Ok(()) => println!("plot done"),
        Err(_) => println!("plot error")

    };

 }

