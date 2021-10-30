
use mads::scene::scenario::Scenario;
use mads::simulator::Simulator;
use mads::ecs::resources::*;
use mads::log::{LogDataType, Logger};

use crate::plot::plot_trajectory_3d;
use crate::logger::FormFlightLogger;

pub fn post_process<T: Scenario>(simulator: &Simulator<T>) {

    // TODO: safely unwrap resources.get()
    let time_history = simulator.state.resources.get::<SimulationTimeHistory>().unwrap();
    let result = simulator.state.resources.get::<SimulationResult>().unwrap();

    let logger = FormFlightLogger;
    if let Err(err) = logger.to_csv(&simulator.state, "./results.csv", LogDataType::SimResult) {
        println!("csv write error, {}", err);
    };

    if let Err(err) = logger.assignments_to_json(&simulator.state, "./assignments.json") {
        println!("json write error, {}", err);
    };

    if let Err(err) = logger.sim_id_to_json(&simulator.state, "./entities.json") {
        println!("json write error, {}", err);
    };

    // (optional)
    match plot_trajectory_3d(&time_history, &result) {

        Ok(()) => println!("plot done"),
        Err(_) => println!("plot error")

    };

}
