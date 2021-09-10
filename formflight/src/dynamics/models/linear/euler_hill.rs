
use na::{DMatrix, DVector};
use crate::dynamics::linear_system::LinearSystem;
use crate::dynamics::statespace::StateSpaceRepresentation;


// put the rest in a the structure
pub fn ClohessyWiltshire(t: f32, x: &DVector<f32>) -> DVector<f32> {

    // Initial conditions
    let x0 = x[0];
    let y0 = x[1];
    let z0 = x[2];
    let xdot0 = x[3];
    let ydot0 = x[4];
    let zdot0 = x[5];

    let n: f32 = 0.00113; // LEO orbit - omega_dot
    // let R: f32 = 405.0 + 6870.0;
    // let mu = 398600.5;
    // let n = (mu/(R.powf(3.0))).sqrt();
    let tau = n*t;

    let cos_nt = tau.cos();
    let sin_nt = tau.sin();

    let xt = (4. - 3.*cos_nt)*x0 + (1./n)*sin_nt*xdot0 + (2./n)*(1.-cos_nt)*ydot0;

    let yt = 6.*(sin_nt - tau)*x0 + y0 - (2./n)*(1.-cos_nt)*xdot0 + (1./n)*(4.*sin_nt - 3.*tau)*ydot0;

    let zt = cos_nt*z0 + (1./n)*sin_nt*zdot0;

    let xtdot = 3.*n*sin_nt*x0 + cos_nt*xdot0 + 2.*sin_nt*ydot0;

    let ytdot = -6.*n*(1.-cos_nt)*x0 - 2.*sin_nt*xdot0 + (4.*cos_nt - 3.)*ydot0;

    let ztdot = -n*sin_nt*z0 + cos_nt*zdot0;

    DVector::<f32>::from_vec(vec![xt, yt, zt, xtdot, ytdot, ztdot])

}


#[derive(Debug, Clone, PartialEq)]
pub struct EulerHill3D {

    pub dynamics: LinearSystem

}

impl EulerHill3D {

    pub fn new() -> Self {

        // let r: f32 = 42164E3; // geostationary orbit
        let r: f32 = 100E3; // LEO
        let mu: f32 = 3.986004418E14;

        let _n = (mu/r.powf(3f32)).sqrt();

        let n: f32 = 0.00113;

//         let A = DMatrix::from_row_slice(
//             6, 6,
//             &[
//                 0., 0., 0., 1., 0., 0.,
//                 0., 0., 0., 0., 1., 0.,
//                 0., 0., 0., 0., 0., 1.,
//                 3.0*n.powf(2f32), 0., 0., 0., 2.0*n, 0.,
//                 0., 0., 0., -2.0*n, 0., 0.,
//                 0., 0., -n.powf(2f32), 0., 0., 0.
//             ]
//         );

        // clohessy-whiltshire
        let A = DMatrix::from_row_slice(
            6, 6,
            &[
                4., 0., 0., 1., 2.0/n, 0.,
                0., 1., 0., -2.0/n, 1., 0.,
                0., 0., 0., 0., 0., 1.,
                3.0*n.powf(2f32), 0., 0., 0., 2.0*n, 0.,
                -6.0*n, 0., 0., -2.0*n, -3., 0.,
                0., 0., -n.powf(2f32), 0., 0., 0.
            ]
        );

        let B = DMatrix::from_row_slice(
            6, 3, 
            &[
                0., 0., 0.,
                0., 0., 0.,
                0., 0., 0.,
                1., 0., 0.,
                0., 1., 0.,
                0., 0., 1.
            ]
        );

        let C = A.clone();
        let D = B.clone();

        let dynamics = LinearSystem::new(A, B, C, D);

        Self { dynamics }

    }



}

impl StateSpaceRepresentation for EulerHill3D {

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
    fn test_ClohessyWiltshire() {

        let x = DVector::from_vec(vec![0., 0., 0., 0.1, 0., 0.]);

        let t0 = 0.0;
        let dt = 20.0;
        let mut t = t0;
        let mut traj = Vec::new();
        for i in 0..10000 {

            let test = ClohessyWiltshire(t, &x);

            traj.push(test);
            t = dt*i as f32;

        }

        // for ele in y.iter() {
        for ele in traj.iter() {
            println!("{:?}", ele.data);
        }

    }
}
