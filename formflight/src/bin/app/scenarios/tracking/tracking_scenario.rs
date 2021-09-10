#![allow(non_snake_case)]

use std::collections::HashMap;
use nalgebra::{DMatrix, DVector};
use legion::*;
use uuid::Uuid;

use formflight::scene::scenario::Scenario;
use formflight::ecs::systems::simple_systems::*;
use formflight::ecs::systems::dynamics_systems::*;
use formflight::ecs::components::*;
use formflight::ecs::resources::*;
use formflight::dynamics::statespace::{State, StateSpace};
use formflight::dynamics::models::linear::double_integrator::*;
use formflight::controls::models::lqr::LinearQuadraticRegulator;

use crate::scenarios::tracking::components::{Agent, Target};
use crate::scenarios::tracking::resources::{NumAgents, NumTargets, Assignment, AssignmentHistory};
use crate::scenarios::tracking::error_dynamics_systems::*;
use crate::distributions::*;
use crate::scenarios::tracking::assignments::ot_assignment;

pub struct TrackingScenario {

    pub num_agents: u32,
    pub num_targets: u32,
    pub agent_formation: Distribution,
    pub target_formation: Distribution

}

impl TrackingScenario {

    pub fn new(num_agents: u32, num_targets:u32) -> Self {

        let agent_formation = Distribution::Sphere;
        let target_formation = Distribution::Circle3D;

        Self {
            num_agents,
            num_targets,
            agent_formation,
            target_formation
        }

    }

    pub fn default() -> Self {

        let agent_formation = Distribution::Circle3D;
        let target_formation = Distribution::Sphere;

        Self {
            num_agents: 2,
            num_targets: 2,
            agent_formation,
            target_formation
        }

    }

    fn setup_agents(&self, world: &mut World, resources: &mut Resources) {

        let radius = 10f32;
        let mut storage = resources.get_mut::<SimulationResult>().unwrap();

        // Generate initial states
        let formation = match self.agent_formation {
            Distribution::Circle2D => circle_3d(radius, self.num_agents), // force 3d scenario
            Distribution::Circle3D => circle_3d(radius, self.num_agents),
            Distribution::Sphere => sphere(radius, self.num_agents)
        };

        // For now just use a double integrator and LQR
        let double_integrator = DoubleIntegrator3D::new();
        let A = double_integrator.dynamics.A.clone();
        let B = double_integrator.dynamics.B.clone();
        let Q = DMatrix::<f32>::identity(6, 6);
        let R = DMatrix::<f32>::identity(3, 3);

        // Define agent components
        let agent_components: Vec<(FullState, DynamicsModel::<DoubleIntegrator3D>, LQRController, SimID, Agent)> = (0..self.num_agents).into_iter()
            .zip(formation.iter())
            .map(| (i, pose) | -> (FullState, DynamicsModel::<DoubleIntegrator3D>, LQRController, SimID, Agent) {

                let name = "Agent".to_string() + &i.to_string();
                let id = Uuid::new_v4();
                let sim_id = SimID { uuid: id, name };

                // Initial conditions
                let state = DVector::<f32>::from_vec(vec![pose.0, pose.1, pose.2, 0.0, 0.0, 0.0]);
                let statespace = StateSpace{
                    position: State::Position{x: 0, y: 1, z: Some(2)},
                    velocity: State::Velocity{x: 3, y: 4, z: Some(5)},
                    attitude: State::Empty,
                    angular_velocity: State::Empty
                };
                let fullstate = FullState { data: state, statespace };

                // Agent dynamics model
                let dynamics = DynamicsModel { model: DoubleIntegrator3D::new() };

                // Agent controller
                let controller = LQRController { model: LinearQuadraticRegulator::new(A.clone(), B.clone(), Q.clone(), R.clone()) };

                let agent_flag = Agent { 0: true };

                (fullstate, dynamics, controller, sim_id, agent_flag)
            })
            .collect();

        // Add agents to storage resource
        for agent in agent_components.iter() {
            storage.data.entry(agent.3.clone()).or_insert(vec![agent.0.clone()]);
        }

        // Generate Agent Entities defined by component tuples and add to the World
        let _agents: &[Entity] = world.extend(agent_components);

    }

    fn setup_targets(&self, world: &mut World, resources: &mut Resources) {

        let mut storage = resources.get_mut::<SimulationResult>().unwrap();
        let mut targetable_set = resources.get_mut::<TargetableSet>().unwrap();

        // Generate initial states
        let radius = 10f32;
        let mut formation = match self.target_formation{
            Distribution::Circle2D => circle_3d(radius, self.num_targets), // force 3d scenario
            Distribution::Circle3D => circle_3d(radius, self.num_targets),
            Distribution::Sphere => sphere(radius, self.num_targets)
        };

        // shift the distribution over
        for (x, _y, _z) in formation.iter_mut() {
            *x += 50.0;
        }

        // For now just use a double integrator and LQR
        let double_integrator = DoubleIntegrator3D::new();
        let A = double_integrator.dynamics.A.clone();
        let B = double_integrator.dynamics.B.clone();
        let Q = DMatrix::<f32>::identity(6, 6);
        let R = DMatrix::<f32>::identity(3, 3);

        // Define target components
        let target_components: Vec<(FullState, DynamicsModel::<DoubleIntegrator3D>, LQRController, SimID, Target)>
            = (0..self.num_targets).into_iter()
            .zip(formation.iter())
            .map(| (i, pose) | -> (FullState, DynamicsModel::<DoubleIntegrator3D>, LQRController, SimID, Target) {

                let name = "Target".to_string() + &i.to_string();
                let id = Uuid::new_v4();
                let sim_id = SimID { uuid: id, name };

                // Initial conditions
                let state = DVector::<f32>::from_vec(vec![pose.0, pose.1, pose.2, 0.0, 0.0, 0.0]);
                let statespace = StateSpace{
                    position: State::Position{x: 0, y: 1, z: Some(2)},
                    velocity: State::Velocity{x: 3, y: 4, z: Some(5)},
                    attitude: State::Empty,
                    angular_velocity: State::Empty
                };
                let fullstate = FullState { data: state, statespace };

                // Target dynamics
                let dynamics = DynamicsModel { model: DoubleIntegrator3D::new() };

                // Target controllers
                let controller = LQRController { model: LinearQuadraticRegulator::new(A.clone(), B.clone(), Q.clone(), R.clone()) };

                // Identifier flag
                let target_flag = Target { 0: true };

                (fullstate, dynamics, controller, sim_id, target_flag)

            })
            .collect();

        // Add targets to storage resource
        // Add targets to targetable set resource
        for target in target_components.iter() {
            storage.data.entry(target.3.clone()).or_insert(vec![target.0.clone()]);
            targetable_set.0.entry(target.3.uuid.clone()).or_insert(target.0.clone());
        }

        // Generate Target Entities defined by component tuples and add to the World
        let _targets: &[Entity] = world.extend(target_components);

    }

    /// Keeps track of Entities that have a Target component
    fn update_targetable_set(&self, world: &mut World, resources: &mut Resources) {

        // query for 'Targetable' components
        let mut targetable_set_atomic = resources.get_mut::<TargetableSet>().unwrap();

        let mut query = <(&SimID, &FullState, &Target)>::query();
        for chunk in query.iter_chunks_mut(world) {

            // we can iterate through a tuple of component references per entity
            for (id, state, target) in chunk {
                if target.0 == true {
                    // update states for targets in target set
                    targetable_set_atomic.0.insert(id.uuid, state.clone());
                    // targetable_set_atomic.0.entry(id.uuid).or_insert(state.clone()) = *state.clone();
                }
            }

        }

    }

    /// Generates an assignment between Agent and Target Entitites based off of position
    fn assign(&self, world: &mut World, resources: &mut Resources) {

        let assignment: Vec<Vec<u32>>;

        // Resources
        let mut assignment_history = resources.get_mut::<AssignmentHistory>().unwrap();
        let mut current_assignment = resources.get_mut::<Assignment>().unwrap();

        // Query entities
        let mut target_query = <(&SimID, &FullState, &Target)>::query();
        let mut agent_query = <(&SimID, &FullState, &Agent)>::query();

        // Agent entity positions and ids
        let mut agent_states: Vec<Vec<f32>> = Vec::new();
        let mut agent_ids: Vec<&Uuid> = Vec::new();
        for (id, state, _agent) in agent_query.iter(world) {
            let pose = vec![state.data[0], state.data[1], state.data[2]];
            agent_states.push(pose);
            agent_ids.push(&id.uuid);
        }

        // Target entity positions and ids
        let mut target_states: Vec<Vec<f32>> = Vec::new();
        let mut target_ids: Vec<&Uuid> = Vec::new();
        for (id, state, _target) in target_query.iter(world) {
            let pose = vec![state.data[0], state.data[1], state.data[2]];
            target_states.push(pose);
            target_ids.push(&id.uuid);
        }

        // Perform assignment of agents to targets
        assignment = match ot_assignment(&agent_states, &target_states) {

            Ok(ot_matrix) => ot_matrix,
            Err(error) => panic!("EMD assignment error {:?}", error)

        };

        // Update AssignmentHistory resource
        for (i, agent) in assignment.iter().enumerate() {
            for (j, possible_target) in agent.iter().enumerate() {
                if *possible_target == 1 {
                    let agent_id = agent_ids[i];
                    let target_id = target_ids[j];
                    assignment_history.map.entry(*agent_id).or_insert(vec![*target_id]).push(*target_id);
                }
            }
        }

        // Update current Assignment resource
        for (_i, (agent_id, _agent_state, _agent)) in agent_query.iter(world).enumerate() {

            let target_id = match assignment_history.map.get(&agent_id.uuid) {
                Some(uuid_list) => uuid_list[uuid_list.len()-1],
                None => continue
            };

            for (_j, (id, target_state, _target)) in target_query.iter(world).enumerate() {

                if id.uuid == target_id {

                    *current_assignment.map.entry(agent_id.uuid).or_insert(None) = Some(target_state.data.clone());

                }

            }

        }

        // println!("{:?}", assignment);


    }

}

impl Scenario for TrackingScenario {

    /// Generate Resources for the scenario and insert into Simulator Resource pool
    fn setup(&self,
        world: &mut World,
        resources: &mut Resources,
    )
    {
        // scenario resources
        let num_agents = NumAgents(self.num_agents);
        let num_targets = NumTargets(self.num_targets);
        let targetable_set = TargetableSet(HashMap::new());
        let assignment = Assignment{ map: HashMap::new() };
        let assignment_history = AssignmentHistory{ map: HashMap::new() };
        let storage = SimulationResult{ data: HashMap::new() };
        resources.insert(num_agents);
        resources.insert(num_targets);
        resources.insert(targetable_set);
        resources.insert(assignment);
        resources.insert(assignment_history);
        resources.insert(storage);

        self.setup_agents(world, resources);
        self.setup_targets(world, resources);

    }

    /// Define systems to be run per loop iteration and return a Schedule to execute
    fn build(&self) -> Schedule {

        let schedule = Schedule::builder()
            // .add_system(print_id_system())
            // .add_system(print_state_system())
            // .add_system(dynamics_lqr_solver_system::<DoubleIntegrator3D>()) // can add any dynamics type here
            .add_system(error_dynamics_lqr_solver_system::<DoubleIntegrator3D>())
            .add_system(update_result_system())
            .add_system(increment_time_system())
            .build();

        schedule

    }

    /// Update and perform logic specific to the scenario
    fn update(&mut self, world: &mut World, resources: &mut Resources) {

        // Updates entities flagged as Targetable
        self.update_targetable_set(world, resources);

        // Perform assignment of Agents to Targets
        self.assign(world, resources);

    }

}


