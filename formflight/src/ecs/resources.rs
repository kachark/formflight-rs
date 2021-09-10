
use std::collections::HashMap;
use na::DVector;
use uuid::Uuid;
use serde::Serialize;

use crate::ecs::components::*;
use crate::math::integrators::IntegratorType;

// Define Engine resources for Legion Entity-Component-System

/// Current simulation time
#[derive(Default)]
pub struct SimulationTime(pub f32);

/// Simulation time history
#[derive(Default, Debug)]
pub struct SimulationTimeHistory {
    pub data: Vec<f32>
}

/// Max simulation time
#[derive(Default)]
pub struct MaxSimulationTime(pub f32);

/// Simulation engine step size
#[derive(Default)]
pub struct EngineStep(pub f32);

/// Integrator step size (ie. dynamics integrations)
#[derive(Default)]
pub struct IntegratorStep(pub f32);

/// Integrator type to be used in simulation
#[derive(Default)]
pub struct Integrator(pub IntegratorType);

/// Tracks a set of entities defined by Uuid
#[derive(Default, Debug)]
pub struct TargetableSet(pub HashMap::<Uuid, FullState>);

/// Storage for Simulation outputs
#[derive(Default, Debug, Clone, Serialize)]
pub struct SimulationResult {
    pub data: HashMap<SimID, Vec<FullState>>
}
