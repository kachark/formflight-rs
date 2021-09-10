
use na::{Rotation3, Vector3, Matrix3};

pub enum CoordinateFrame {
    World { x: f32, y: f32, z: f32 },
    BodyFixed { xB: f32, yB: f32, zB: f32 }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartesianFrame {

    origin: Vector3<f32>,
    axes: Matrix3<f32>

}

impl CartesianFrame {

    pub fn new(origin: (f32, f32, f32)) -> Self {

        let o = Vector3::new(origin.0, origin.1, origin.2);
        let mut axes = Matrix3::from_element(0.0);
        axes.fill_with_identity();

        Self { origin: o, axes }

    }

    pub fn default() -> Self {

        let origin = Vector3::new(0f32, 0f32, 0f32);
        let mut axes = Matrix3::from_element(0.0);
        axes.fill_with_identity();

        Self { origin, axes }

    }

    /// Rotates frame using euler angles rotations with a ZYX sequence
    pub fn rotate(&mut self, roll: f32, pitch: f32, yaw: f32) {

        let rot = Rotation3::from_euler_angles(roll, pitch, yaw);

        // self.axes = rot.transpose() * self.axes // transpose because nalgebra uses col-major matrices
        self.axes = rot * self.axes

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_CartesianFrame_rotate() {

        let mut body_fixed_frame = CartesianFrame::default();
        body_fixed_frame.rotate(std::f32::consts::FRAC_PI_4, 0.0, std::f32::consts::FRAC_PI_6);

        let correct = Matrix3::from_row_slice(
            &[1.0, 0.0, 0.0,
            0.0, 0.707, -0.707,
            0.0, 0.707, 0.707]);

        println!("{:?}", body_fixed_frame);

        let _ = relative_eq!(body_fixed_frame.axes, correct);
    }

}

