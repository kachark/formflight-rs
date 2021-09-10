
use std::fmt;

use nalgebra::DVector;
use uuid::Uuid;
use serde::Serialize;

use crate::controls::models::lqr::LinearQuadraticRegulator;
use crate::dynamics::statespace::{StateSpace, StateSpaceRepresentation};
use crate::math::coordinate_frame::CartesianFrame;

// Define Components
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/// Define coordinates frame for an entity
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartesianCoordinates {
    pub frame: CartesianFrame
}

/// Assigns a Dynamic Vector (see nalgebra) to an Entity
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FullState {
    pub data: DVector<f32>,
    pub statespace: StateSpace
}

/// Assigns a unique identifier and name to an Entity
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SimID {
    pub uuid: Uuid,
    pub name: String
}

impl fmt::Display for SimID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uuid: {}, name: {}", self.uuid, self.name)
    }
}

/// Flags an Entity as Targetable
/// Targetable entities are tracked within the TargetableSet
/// resource
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Targetable(pub bool);

/// Generic Component for a Dynamics model
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicsModel<T>
where
    T: StateSpaceRepresentation
{
    pub model: T
}

/// Component representing a Linear-Quadratic Regulator
#[derive(Clone, Debug, PartialEq)]
pub struct LQRController {
    pub model: LinearQuadraticRegulator
}



