#![allow(non_snake_case)]
#[macro_use]

extern crate approx;
extern crate lapack;
extern crate lapack_src;
extern crate nalgebra as na;
extern crate serde;

// FormFlight Core
pub mod controls;
pub mod dynamics;
pub mod math;
pub mod log;
pub mod util;

// FormFlight Engine
pub mod simulator;
pub mod scene;
pub mod ecs;

#[cfg(test)]
mod tests {

    #[test]
    fn test_dynamics() {

        use super::math::ivp_solver::euler::MidPointEuler;
        use super::controls::models::lqr::LinearQuadraticRegulator as LQR;
        use super::dynamics::models::linear::double_integrator::DoubleIntegrator2D;
        use super::dynamics::statespace::StateSpaceRepresentation;
        use super::na::{DMatrix, DVector};

        let model = DoubleIntegrator2D::new();

        // Define matrices for a Linear Quadratic Regulator
        let Q = DMatrix::<f32>::identity(4, 4);
        let R = DMatrix::<f32>::identity(2, 2);
        let lqr = LQR::new(
            model.dynamics.A.clone(),
            model.dynamics.B.clone(),
            Q,
            R,
        );


        // Define initial conditions
        // let x0 = DVector::from_vec(vec![10., 10.]);
        let x0 = DVector::from_vec(vec![10., 10., -10., -30.]);

        // Solve the LQR controller
        let (K, _P) = match lqr.solve() {
            Ok((value1, value2)) => (value1, value2),
            Err(_) => (DMatrix::<f32>::zeros(1, 1), DMatrix::<f32>::zeros(1, 1)),
        };

        // Simulate
        let mut trajectory: Vec<DVector<f32>> = vec![x0];
        let step = 0.1;
        let t0 = 0;
        let tf = 10;
        for t in t0..tf {

            let x_prev = trajectory[trajectory.len()-1].clone();

            // Wrap dynamics/controls in appropriately defined closure - f(t, x)
            let f = |t: f32, x: &DVector<f32>| {
                let u = -&K * x;
                model.f(t, x, Some(&u))
            };

            // Integrate dynamics
            let (_t_history, traj) = MidPointEuler(f, t as f32, x_prev, (t as f32)+1f32, step);

            // Store result
            for state in traj {

                trajectory.push(state);

            }

        }

        for x in trajectory {

            println!("{:?}", &x.data);

        }

    }
}
