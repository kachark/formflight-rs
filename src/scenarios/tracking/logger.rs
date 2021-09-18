
use std::fs;
use std::io::BufWriter;
use std::error::Error;
use std::collections::HashMap;
use legion::*;

use mads::dynamics::statespace::StateSpace;
use mads::ecs::components::{SimID, FullState};
use mads::ecs::resources::{SimulationResult, SimulationTimeHistory};
use mads::simulator::state::SimulationState;
use mads::log::simulation_logger::SimulationLogger;

use crate::scenarios::tracking::resources::AssignmentHistory;
use crate::scenarios::tracking::components::{Agent, Target};

pub struct Logger;

impl Logger {

    /// Write Agent-to-Target assignments over time to JSON
    pub fn assignments_to_json(&self, sim_state: &SimulationState) -> serde_json::Result<()> {

        let f = fs::File::create("assignments.json").expect("Unable to create file");
        let bw = BufWriter::new(f);

        // Access time and stored assignments
        // let time_history = sim_state.resources.get::<SimulationTimeHistory>().unwrap();
        let assignments = sim_state.resources.get_mut::<AssignmentHistory>().unwrap();

        // Serialize hashmap of assignment history to JSON
        let j = serde_json::to_string_pretty(&assignments.map)?;

        // println!("{}", j);

        serde_json::to_writer(bw, &j).expect("Failed writing : (");

        Ok(())

    }

    /// Write Agent/Target SimID and corresponding model Statespace to JSON
    pub fn sim_id_to_json(&self, sim_state: &SimulationState) -> serde_json::Result<()> {

        let f = fs::File::create("entities.json").expect("Unable to create file");
        let bw = BufWriter::new(f);

        let mut agent_id_query = <(&SimID, &FullState, &Agent)>::query();
        let mut target_id_query = <(&SimID, &FullState, &Target)>::query();

        let mut entities = HashMap::<String, Vec<(&SimID, &StateSpace)>>::new();

        for (id, state, _agent) in agent_id_query.iter(&sim_state.world) {
            entities.entry("Agents".to_string()).or_insert(Vec::new()).push( (&id, &state.statespace) );
        }

        for (id, state, _target) in target_id_query.iter(&sim_state.world) {
            entities.entry("Targets".to_string()).or_insert(Vec::new()).push( (&id, &state.statespace) );
        }

        // Serialize to JSON
        let entities_json = serde_json::to_string_pretty(&entities)?;

        // println!("{}", &entities_json);

        serde_json::to_writer(bw, &entities_json).expect("Failed writing : (");


        Ok(())

    }

}

impl SimulationLogger for Logger {

    /// Write SimulationTimeHistory and SimulationResult Resources to CSV
    fn to_csv(&self, sim_state: &SimulationState) -> std::result::Result<(), Box<dyn Error>> {

        let mut wtr = csv::Writer::from_path("test.csv")?;

        // Access stored time and entity FullStates
        let time_history = sim_state.resources.get::<SimulationTimeHistory>().unwrap();
        let results = sim_state.resources.get::<SimulationResult>().unwrap();

        // println!("{:?}", time_history);

        // Construct header
        let mut header: Vec<String> = vec!["Time".to_string()];
        for (id, fullstate_history) in results.data.iter() {

            let dx = fullstate_history[0].data.len();
            for _ in 0..dx {
                header.push(id.name.clone());
            }

        }

        wtr.write_record(&header)?;

        // Go row-by-row and serialize time and header aligned state values
        for (k, time) in time_history.data.iter().enumerate() {

            let mut row: Vec<f32> = vec![*time];
            for (_id, state_history) in results.data.iter() {
                let fullstate_k = &state_history[k];
                for dim in fullstate_k.data.iter() {
                    row.push(*dim);
                }
            }

            wtr.serialize(row)?;

        }

        wtr.flush()?;

        Ok(())

    }

}


