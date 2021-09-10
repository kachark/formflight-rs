
use na::{DVector, DMatrix};
use crate::dynamics::linear_system::LinearSystem;
use crate::dynamics::statespace::StateSpaceRepresentation;

/// Linearized inverted pendulum
/// Small angle approximations and linearized about vertically upward angle
/// theta = pi
/// https://ctms.engin.umich.edu/CTMS/index.php?example=InvertedPendulum&section=SystemModeling
#[derive(Debug, Clone, PartialEq)]
pub struct InvertedPendulum {

    pub dynamics: LinearSystem

}

impl InvertedPendulum {

    pub fn new() -> Self {

        let A = DMatrix::from_row_slice(4, 4, 
                    &[0., 1., 0., 0.,
                      0., -0.1818, 2.6727, 0.,
                      0., 0., 0., 1.,
                      0., -0.4545, 31.1818, 0.]
        );

        let B = DMatrix::from_row_slice(4, 1, 
                    &[0., 1.8182, 0., 4.5455]);

        let C = DMatrix::from_row_slice(2, 4,
                    &[1., 0., 0., 0.,
                      0., 0., 1., 0.,]
        );

        let D = DMatrix::from_row_slice(2, 1, &[0., 0.]);

        let dynamics = LinearSystem::new(A, B, C, D);

        Self { dynamics }

    }

}

impl StateSpaceRepresentation for InvertedPendulum {

    fn f(&self, t: f32, x: &DVector<f32>, u: Option<&DVector<f32>>) -> DVector<f32> {

        self.dynamics.f(t, x, u)

    }

    fn h(&self, t: f32, x: &DVector<f32>, u: Option<&DVector<f32>>) -> DVector<f32> {

        self.dynamics.h(t, x, u)

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Linearized_InvertedPendulum() {

        let model = InvertedPendulum::new();
        // x, x_dot, phi, phi_dot (phi is some small deviation from equilibrium)
        let x0 = DVector::from_vec(vec![10., 10., 10., 10.]);

        let xdot = model.f(0f32, &x0, None);

        println!("{:?}", xdot);
    }
}
