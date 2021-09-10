
use legion::{World, Resources};
use uuid::Uuid;
use crate::util::misc::range_step;
use crate::ecs::resources::*;
use crate::ecs::components::{CartesianCoordinates, SimID};
use crate::math::coordinate_frame::CartesianFrame;
use crate::simulator::configuration::{EngineConfig, SimulationConfig};

/// Define World, Resources, and Components
/// Input:
/// Engine and Simulation parameters
/// engine_params - &HashMap<String, EngineParameter>
/// sim_params - &HashMap<String,  SimulationParameter>
/// Output:
/// World, Resources and time history of the simulation
/// (legion::World, legion::Resources, Vec<f32>)
pub fn setup(
    engine_params: &EngineConfig,
    sim_params: &SimulationConfig
) -> (World, Resources, Vec<f32>)
{

    // Declare World and Resources
    let mut world = World::default();
    let mut resources = Resources::default();

    // Generate Engine time values
    let times = get_times(engine_params);

    // Core simulation Entities
    add_simulation_entities(&mut world);

    // Populate resources
    add_engine_resources(&mut resources, engine_params);
    add_simulation_resources(&mut resources, sim_params);

    (world, resources, times)

}

/// Inserts Resources to ECS derived from an EngineConfig
fn add_engine_resources(resources: &mut legion::Resources, config: &EngineConfig) {

    resources.insert(SimulationTime(config.simulation_time));
    resources.insert(SimulationTimeHistory{ data: vec![config.simulation_time] });
    resources.insert(MaxSimulationTime(config.max_simulation_time));
    resources.insert(EngineStep(config.engine_step));

}

/// Inserts core Entities for simulation
fn add_simulation_entities(world: &mut legion::World) {

    // Add entity for origin
    let inertial_frame = CartesianCoordinates {
        frame: CartesianFrame::default()
    };
    let sim_id = SimID { uuid: Uuid::new_v4(), name: "O".to_string() };
    world.extend(vec![(inertial_frame, sim_id)]);

}

/// Inserts Resources to ECS derived from a SimulationConfig
fn add_simulation_resources(resources: &mut legion::Resources, config: &SimulationConfig) {

    resources.insert(IntegratorStep(config.integrator_step));
    resources.insert(Integrator(config.integrator));

}

/// Generate Vector of values for the time history of the simulation from an EngineConfig
fn get_times(config: &EngineConfig) -> Vec<f32> {

    let start_time = config.simulation_time;
    let max_time = config.max_simulation_time;
    let step = config.engine_step;

    range_step(start_time, max_time, step)

}

