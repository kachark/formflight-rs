
use na::DVector;
use serde::Serialize;

/// Defines an interface for solving systems of equations according to a State Space
/// model
pub trait StateSpaceRepresentation {
    fn f(&self, t: f32, x: &DVector<f32>, u: Option<&DVector<f32>>) -> DVector<f32>;
    fn h(&self, t: f32, x: &DVector<f32>, u: Option<&DVector<f32>>) -> DVector<f32>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum State {
    Empty,
    Position{ x: u32, y: u32, z: Option<u32> },
    Velocity{ x: u32, y: u32, z: Option<u32> },
    Attitude{ psi: u32, theta: u32, phi: Option<u32> },
    AngularVelocity{ psi: u32, theta: u32, phi: Option<u32> },
}

/// Defines a generic statespace for dynamic mechanical systems in a Cartesian coordinate system
/// Each field represents the corresponding indices of the state vector
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub struct StateSpace {
    pub position: State,
    pub velocity: State,
    pub attitude: State,
    pub angular_velocity: State
}
