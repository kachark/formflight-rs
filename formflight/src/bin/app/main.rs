#[allow(non_snake_case)]
extern crate plotters;
extern crate rust_optimal_transport as rot;

pub mod plot;
pub mod distributions;
pub mod scenarios;

use formflight::simulator::configuration::{EngineConfig, SimulationConfig};
use formflight::simulator::simulation::Simulation;
use formflight::simulator::state::SimulationState;
use formflight::ecs::resources::*;
// use formflight::log::logger::Logger;
use formflight::log::simulation_logger::SimulationLogger;

// use formflight::scene::scenario::SimpleScenario;
use crate::scenarios::tracking::tracking_scenario::TrackingScenario;
use crate::scenarios::tracking::resources::AssignmentHistory;

use crate::scenarios::tracking::logger::Logger;

fn main() {

    // Configure simulation, engine, and scenario
    let engine_config = EngineConfig::default();
    let sim_config = SimulationConfig::default();
    let sim_state = SimulationState::new(engine_config, sim_config);
    let scenario = TrackingScenario::default();
    // let scenario = SimpleScenario::new();

    let mut simulation = Simulation::new(sim_state, scenario);
    simulation.build();
    simulation.run();

    // NOTE: post processing
    // TODO: safely unwrap resources.get()
    let time_history = simulation.state.resources.get::<SimulationTimeHistory>().unwrap();
    let result = simulation.state.resources.get::<SimulationResult>().unwrap();


    // TODO: need to have a logger for scenario specific things - maybe a SimLogger trait
    // tracking scenario specific
    // let targetable_set_atomic = simulation.state.resources.get_mut::<TargetableSet>().unwrap();
    // let assignments_atomic = simulation.state.resources.get_mut::<AssignmentHistory>().unwrap();

//     for (uuid, trajectory) in result.data.iter() {
//         println!("Entity: {:?}", uuid);
//         println!("length: {:?}", trajectory.len());
//         for state in trajectory {
//             println!("{:?}", state);
//         }
//     }

    // println!("{:?}", time_history.data.len());
    // for t in time_history.data.iter() {

    //     println!("{:?}", t);

    // }

    // println!("{:?}", targetable_set_atomic);

    // // assignments
    // for (agent_uuid, target_uuids) in assignments_atomic.map.iter() {

    //     println!("Agent id: {}", agent_uuid);
    //     println!("{:?}", target_uuids);

    // }

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

    match plot::plot_trajectory_3d(&time_history, &result) {

        Ok(()) => println!("hi"),
        Err(_) => println!("plot error")

    };

 }

