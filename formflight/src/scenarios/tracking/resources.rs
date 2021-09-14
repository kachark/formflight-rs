
use nalgebra::DVector;

use std::collections::HashMap;
use uuid::Uuid;
use serde::Serialize;

/// Number of Agents
#[derive(Default)]
pub struct NumAgents(pub u32);

/// Number of Targets
#[derive(Default)]
pub struct NumTargets(pub u32);

/// Number of Obstacles
#[derive(Default)]
pub struct NumObstacles(pub u32);

/// Assignment history between agents and targets
#[derive(Default, Debug, Serialize)]
pub struct AssignmentHistory {
    pub map: HashMap<Uuid, Vec<Uuid>>
}

/// Current assignment as a mapping from Agent uuid to Target state
#[derive(Default, Debug)]
pub struct Assignment {
    pub map: HashMap<Uuid, Option<DVector<f32>>>
}
