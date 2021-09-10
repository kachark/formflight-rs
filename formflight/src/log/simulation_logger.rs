
use std::error::Error;

use crate::simulator::state::SimulationState;

pub trait SimulationLogger {

    fn to_csv(&self, sim_state: &SimulationState) -> Result<(), Box<dyn Error>>;

}

