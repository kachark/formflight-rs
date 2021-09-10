
use na::{DMatrix, DVector};
use crate::dynamics::nonlinear_system::NonlinearSystem;
use crate::dynamics::statespace::StateSpaceRepresentation;

fn equations_of_motion(_t: f32, x: &DVector<f32>, u: Option<&DVector<f32>>) -> DVector<f32> {

    // Cart inverted pendulum model params
    // https://link.springer.com/article/10.1007/s11633-014-0818-1
    let g = 9.81; // gravity
    let l = 5.0; // length of pendulum
    // let k = 0.7; // coeff. of friction
    let m = 3.0; // mass
    let M = 6.0; // mass of cart

    // Pendulum equations
    let mut res = DVector::<f32>::zeros(x.len());

    let x1 = x[0]; // theta
    let x2 = x[1]; // theta_dot
    let _x3 = x[2]; // x
    let x4 = x[3]; // x_dot

    match u {

        Some(u) => {
            let u = u[0];
            res[0] = x2;
            res[1] = (u*x1.cos() - (M+m)*g*x1.sin() + m*l*(x1.cos() * x1.sin())*x2.powf(2.0)) / 
                ( m*l*x1.cos().powf(2.0) - (M+m)*l );
            res[2] = x4;
            res[3] = (u + m*l*(x1.sin())*x2.powf(2.0) - m*g*x1.cos()*x1.sin()) / 
                ( M + m - m*x1.cos().powf(2.0) );
        },

        None => println!("no control input provided")

    };

    res


}

fn output_equations(_t: f32, x: &DVector<f32>, _u: Option<&DVector<f32>>) -> DVector<f32> {

    let C = DMatrix::from_row_slice(2, 4,
            &[1., 0., 0., 0.,
            0., 0., 1., 0.]);

    C*x

}

pub struct InvertedPendulum {

    dynamics: NonlinearSystem::< fn(f32, &DVector<f32>, Option<&DVector<f32>>) -> DVector<f32>,
                fn(f32, &DVector<f32>, Option<&DVector<f32>>) -> DVector<f32> >,

}

impl InvertedPendulum {

    pub fn new() -> Self {

        // explicitly convert Function Item to Function pointer
        let f = equations_of_motion as fn(f32, &DVector<f32>, Option<&DVector<f32>>) -> DVector<f32>;
        let h = output_equations as fn(f32, &DVector<f32>, Option<&DVector<f32>>) -> DVector<f32>;
        let dynamics = NonlinearSystem::new(f, h, 2, 1);

        Self {
            dynamics,
        }

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
    use crate::math::ivp_solver::rk45::RungeKutta45;
    use std::f32::consts::FRAC_PI_4;

    #[test]
    fn test_InvertedPendulum() {

        let pendulum = InvertedPendulum::new();

        // initial conditions
        let x0 = DVector::<f32>::from_vec(vec![FRAC_PI_4, -1.0, 0.0, 0.0]); // omega, omega_dot
        let t0 = 0.0;

        // Integrate the dynamics

        // Wrap model in appropriately defined closure for integrator (ie. f(t,x))
        let dynamics = |t: f32, x: &DVector<f32>| {
            // Some constant control input
            let u = DVector::<f32>::from_vec(vec![0.0]);
            pendulum.f(t, x, Some(&u))
        };

        let tf = 10.0;
        let n = 100.0;
        let step = (tf - t0) / n;
        let rtol = 1E-3;
        let (_t, y) = RungeKutta45(dynamics, t0, x0, tf, step, rtol);

        for ele in y.iter() {
            println!("{:?}", ele.data);
        }

    }

}
