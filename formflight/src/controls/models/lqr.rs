use na::{DMatrix, DVector};
use std::fmt;

use crate::math::riccati::*;

#[derive(Clone, Debug)]
pub struct ControlError;

impl fmt::Display for ControlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Control error")
    }
}

/// Continuous Infinite-Horizon Linear Quadratic Regulator
#[derive(Debug, Clone, PartialEq)]
pub struct LinearQuadraticRegulator {
    A: DMatrix<f32>,
    B: DMatrix<f32>,
    R: DMatrix<f32>,
    Q: DMatrix<f32>,
}

impl LinearQuadraticRegulator {
    pub fn new(A: DMatrix<f32>, B: DMatrix<f32>, Q: DMatrix<f32>, R: DMatrix<f32>) -> Self {
        // TODO assert correct sizes
        assert_eq!(A.shape().0, A.shape().1);
        assert_eq!(A.shape().0, Q.shape().0);
        assert_eq!(A.shape().1, Q.shape().1);
        assert_eq!(B.shape().0, A.shape().0);
        assert_eq!(B.shape().1, R.shape().0);

        Self { A, B, Q, R }
    }

    /// Returns LQR gain and solution to the Continuous Algebraic Riccati Equation
    pub fn solve(&self) -> Result<(DMatrix<f32>, DMatrix<f32>), ControlError> {
        let K: DMatrix<f32>;
        let P: DMatrix<f32>;

        P = match solve_continuous_riccati_eigen(&self.A, &self.B, &self.Q, &self.R) {
            Ok(result) => result,
            Err(LinAlgError) => return Err(ControlError),
        };

        if let Some(Rinv) = &self.R.clone().try_inverse() {
            K = Rinv * &self.B.transpose() * &P;

            Ok((K, P))
        } else {
            return Err(ControlError);
        }
    }


    /// Returns the Cost-to-go for the LQR
    /// V = x^TQx + u^TRu
    pub fn cost_to_go(&self, x: &DVector<f32>, u: &DVector<f32>) -> f32 {

        let xTQx = x.transpose()*(&self.Q*x);
        let uTRu = u.transpose()*(&self.R*u);

        let cost = xTQx + uTRu;

        *cost.get(0).unwrap()

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LinearQuadraticRegulator_solve() {
        // generate row-major matrices
        let A = DMatrix::from_row_slice(2, 2, &[0., 1., 0., 0.]);

        let B = DMatrix::from_row_slice(2, 1, &[0., 1.]);

        let Q = DMatrix::<f32>::identity(2, 2);
        let R = DMatrix::from_vec(1, 1, vec![1.]);

        let controller = LinearQuadraticRegulator::new(A, B, Q, R);

        let (K, _P) = match controller.solve() {
            Ok((value1, value2)) => (value1, value2),
            _ => (DMatrix::<f32>::zeros(2, 2), DMatrix::<f32>::zeros(2, 2)),
        };

        let _P_true = DMatrix::from_row_slice(2, 2, &[3.0_f32.sqrt(), 1., 1., 3.0_f32.sqrt()]);

        let K_true = DMatrix::from_row_slice(1, 2, &[1., 3.0_f32.sqrt()]);

    //     println!("P: {:?}", P);
    //     println!("P_true: {:?}", P_true);
    //     println!("K: {:?}", K);
    //     println!("K_true: {:?}", K_true);

        let _ = relative_eq!(K, K_true);
    }

    #[test]
    fn test_LinearQuadraticRegulator_cost_to_go() {
        // generate row-major matrices
        let A = DMatrix::from_row_slice(2, 2, &[0., 1., 0., 0.]);

        let B = DMatrix::from_row_slice(2, 1, &[0., 1.]);

        let Q = DMatrix::<f32>::identity(2, 2);
        let R = DMatrix::from_vec(1, 1, vec![1.]);

        let controller = LinearQuadraticRegulator::new(A, B, Q, R);

        // Solve the LQR controller
        let (K, _P) = match controller.solve() {
            Ok((value1, value2)) => (value1, value2),
            Err(_) => (DMatrix::<f32>::zeros(1, 1), DMatrix::<f32>::zeros(1, 1)),
        };

        let x = DVector::from_vec(vec![10., 10.]);
        let u = -&K * &x;

        let result = controller.cost_to_go(&x, &u);
        let correct = 946.41016;
        // println!("{:?}", result);

        // let _ = relative_eq!(result, correct);
        assert_relative_eq!(result, correct);

    }
}
