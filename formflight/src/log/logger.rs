
use std::error::Error;

use crate::ecs::resources::{SimulationResult, SimulationTimeHistory};
use crate::simulator::state::SimulationState;
use crate::log::simulation_logger::SimulationLogger;

pub struct Logger;

impl SimulationLogger for Logger {

    fn to_csv(&self, sim_state: &SimulationState) -> Result<(), Box<dyn Error>> {

        let mut wtr = csv::Writer::from_path("test.csv")?;

        // Access time and entity FullStates
        let time_history = sim_state.resources.get::<SimulationTimeHistory>().unwrap();
        let results = sim_state.resources.get::<SimulationResult>().unwrap();

        println!("{:?}", time_history);

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


