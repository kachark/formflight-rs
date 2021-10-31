
use std::fs;
use std::io::BufWriter;
use std::error::Error;
use std::collections::HashMap;
use legion::*;

// MADS
use mads::ecs::components::{SimID, FullState};
use mads::simulator::state::SimulatorState;
use mads::log::Logger;

// formflight
use crate::ecs::resources::AssignmentHistory;
use crate::ecs::components::{Agent, Target};

pub struct FormFlightLogger;

impl FormFlightLogger {

    /// Write Agent-to-Target assignments over time to JSON
    pub fn assignments_to_json(&self, sim_state: &SimulatorState, filepath: &str) -> serde_json::Result<()> {

        let f = fs::File::create(filepath).expect("Unable to create file");
        let bw = BufWriter::new(f);

        // Access time and stored assignments
        // let time_history = sim_state.resources.get::<SimulationTimeHistory>().unwrap();
        let assignments = sim_state.ecs.resources.get_mut::<AssignmentHistory>().unwrap();

        // Serialize hashmap of assignment history to JSON
        let j = serde_json::to_string_pretty(&assignments.map)?;

        // println!("{}", j);

        serde_json::to_writer(bw, &j).expect("Failed writing : (");

        Ok(())

    }

    /// Write Agent/Target SimID and corresponding model Statespace to JSON
    pub fn sim_id_to_json(&self, sim_state: &SimulatorState, filepath: &str) -> serde_json::Result<()> {

        let f = fs::File::create(filepath).expect("Unable to create file");
        let bw = BufWriter::new(f);

        let mut agent_id_query = <(&SimID, &FullState, &Agent)>::query();
        let mut target_id_query = <(&SimID, &FullState, &Target)>::query();

        // let mut entities = HashMap::<String, Vec<(&SimID, &StateSpace)>>::new();
        let mut entities = HashMap::<String, Vec<&SimID>>::new();

        for (id, state, _agent) in agent_id_query.iter(&sim_state.ecs.world) {
            // entities.entry("Agents".to_string()).or_insert(Vec::new()).push( (&id, &state.statespace) );
            entities.entry("Agents".to_string()).or_insert(Vec::new()).push( &id );
        }

        for (id, state, _target) in target_id_query.iter(&sim_state.ecs.world) {
            // entities.entry("Targets".to_string()).or_insert(Vec::new()).push( (&id, &state.statespace) );
            entities.entry("Targets".to_string()).or_insert(Vec::new()).push( &id );
        }

        // Serialize to JSON
        let entities_json = serde_json::to_string_pretty(&entities)?;

        // println!("{}", &entities_json);

        serde_json::to_writer(bw, &entities_json).expect("Failed writing : (");


        Ok(())

    }

}

// Implement Logger for FormFlightLogger and use default to_csv() function
impl Logger for FormFlightLogger {

}


