
use na::DVector;
use crate::util::misc::range_step;

pub fn ForwardEuler<F>(
    f: F,
    t0: f32,
    y0: DVector<f32>,
    tf: f32,
    step: f32
) -> (Vec<f32>, Vec<DVector<f32>>)
where
    F: Fn(f32, &DVector<f32>) -> DVector<f32>,
{
    let h = step;
    let time = range_step(t0, tf, h);

    let mut y: Vec<DVector<f32>> = vec![DVector::<f32>::zeros(y0.len()); time.len()];
    y[0] = y0;

    for k in 0..time.len() - 1 {
        let yk = &y[k];

        let m = h * f(time[k], &yk);

        // Need to clone because y owns the state data
        y[k + 1] = yk + m;
    }

    (time, y)
}


pub fn MidPointEuler<F>(
    f: F,
    t0: f32,
    y0: DVector<f32>,
    tf: f32,
    step: f32
) -> (Vec<f32>, Vec<DVector<f32>>)
where
    F: Fn(f32, &DVector<f32>) -> DVector<f32>,
{
    let h = step;
    let time = range_step(t0, tf, h);

    let mut y: Vec<DVector<f32>> = vec![DVector::<f32>::zeros(y0.len()); time.len()];
    y[0] = y0;

    for k in 0..time.len() - 1 {
        let yk = &y[k];

        let y_half = yk + (h / 2.0 * f(time[k], yk));

        y[k + 1] = yk + (h * f(time[k] + (h/2.0), &y_half));
    }

    (time, y)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ForwardEuler() {

        use na::DMatrix;
        use crate::controls::models::lqr::LinearQuadraticRegulator as LQR;
        use crate::dynamics::models::linear::double_integrator::DoubleIntegrator1D;
        use crate::dynamics::statespace::StateSpaceRepresentation;

        let model = DoubleIntegrator1D::new();

        let Q = DMatrix::<f32>::identity(2, 2);
        let R = DMatrix::from_vec(1, 1, vec![1.]);

        let lqr = LQR::new(
            model.dynamics.A.clone(),
            model.dynamics.B.clone(),
            Q,
            R,
        );

        let y0 = DVector::from_vec(vec![10., -40.]);
        let (K, _P) = match lqr.solve() {
            Ok((value1, value2)) => (value1, value2),
            Err(_) => (DMatrix::<f32>::zeros(1, 1), DMatrix::<f32>::zeros(1, 1)),
        };

        // Wrap dynamics/controls in appropriately defined closure
        let f = |t: f32, x: &DVector<f32>| {
            let u = -&K * x;
            model.dynamics.f(t, x, Some(&u))
        };

        let t0 = 0.0;
        let tf = 10.0;
        let n = 100.0;
        let step = (tf - t0) / n;
        let (_t, y) = ForwardEuler(f, t0, y0, tf, step);

        for ele in y.iter() {
            println!("{:?}", ele.data);
        }
    }

    #[test]
    fn test_MidPointEuler() {

        use na::DMatrix;
        use crate::controls::models::lqr::LinearQuadraticRegulator as LQR;
        use crate::dynamics::models::linear::double_integrator::DoubleIntegrator2D;
        use crate::dynamics::statespace::StateSpaceRepresentation;

        let model = DoubleIntegrator2D::new();

        let Q = DMatrix::<f32>::identity(4, 4);
        let R = DMatrix::<f32>::identity(2, 2);

        let lqr = LQR::new(
            model.dynamics.A.clone(),
            model.dynamics.B.clone(),
            Q,
            R,
        );

        let (K, _P) = match lqr.solve() {
            Ok((value1, value2)) => (value1, value2),
            Err(_) => panic!["LQR solve"],
        };

        let y0 = DVector::from_vec(vec![10., 10., 10., 10.]);

        // Wrap dynamics/controls in appropriately defined closure
        let f = |t: f32, x: &DVector<f32>| {
            let u = -&K * x;
            model.dynamics.f(t, x, Some(&u))
        };

        let t0 = 0.0;
        let tf = 10.0;
        let n = 100.0;
        let step = (tf - t0) / n;
        let (_t, y) = MidPointEuler(f, t0, y0, tf, step);

        for ele in y.iter() {
            println!("{:?}", ele.data);
        }
    }
}

