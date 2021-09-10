
use std::f32::consts::PI as pi;

pub enum Distribution {

    Circle2D,
    Circle3D,
    Sphere

}


/// Computes the x,y position on a circle for a given number of points
///  r: radius of circle
pub fn circle_2d(radius: f32, num_samples: u32) -> Vec<(f32, f32)> {

    let distribution: Vec<(f32, f32)> = (0..num_samples).into_iter()
        .map(|i: u32| -> (f32, f32) {
            let angle = (i as f32) * (2.0*pi) / (num_samples as f32);
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            (x, y)
        })
        .collect();

    distribution

}

/// Computes the x,y position on a circle for a given number of points
///  r: radius of circle
pub fn circle_3d(radius: f32, num_samples: u32) -> Vec<(f32, f32, f32)> {

    let distribution: Vec<(f32, f32, f32)> = (0..num_samples).into_iter()
        .map(|i: u32| -> (f32, f32, f32) {
            let angle = (i as f32) * (2.0*pi) / (num_samples as f32);
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let z = 0f32;
            (x, y, z)
        })
        .collect();

    distribution

}

/// Computes the x,y,z positions on a sphere for a given number of points
/// http://blog.marmakoide.org/?p=1
/// r: radius of sphere / scaling factor
/// nsamples: total number of points on sphere
/// sample: nth point along the sphere
pub fn sphere(r: f32, num_samples: u32) -> Vec<(f32, f32, f32)> {

    let distribution: Vec<(f32, f32, f32)> = (0..num_samples).into_iter()
        .map(|i: u32| -> (f32, f32, f32) {
            let golden_angle = pi * (3f32 - 5f32.sqrt());
            let theta = golden_angle * i as f32;
            let z_i = (1f32 - 1f32/(num_samples as f32)) * 
                        (1f32 - (2f32*(i as f32))/(num_samples as f32 - 1f32));
            let radius = (1f32 - z_i.powf(2f32)).sqrt();
            let x = r * radius * theta.cos();
            let y = r * radius * theta.sin();
            let z = r * z_i;
            (x, y, z)
        })
        .collect();

    distribution

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_2d() {

        let result = circle_2d(10.0, 10);
        let truth = vec![
            (10.0, 0.0),
            (8.097, 5.878),
            (3.090, 9.511),
            (-3.090, 9.511),
            (-8.097, 5.878),
            (-10.0, 0.0),
            (-8.097, -5.878),
            (-3.090, -9.511),
            (3.090, -9.511),
            (8.097, -5.878)
        ];

        println!("{:?}", result);

    }

    #[test]
    fn test_circle_3d() {

        let result = circle_3d(10.0, 10);
        let truth = vec![
            (10.0, 0.0, 0.0),
            (8.097, 0.0, 5.878),
            (3.090, 0.0, 9.511),
            (-3.090, 0.0, 9.511),
            (-8.097, 0.0, 5.878),
            (-10.0, 0.0, 0.0),
            (-8.097, 0.0, -5.878),
            (-3.090, 0.0, -9.511),
            (3.090, 0.0, -9.511),
            (8.097, 0.0, -5.878)
        ];

        println!("{:?}", result);

    }

    #[test]
    fn test_sphere() {
        let result = sphere(10.0, 10);
        let truth = vec![
            (4.359, 0.0, 9.0),
            (-5.266, 4.824, 7.0),
            (0.757, -8.627, 5.0),
            (5.804, 7.570, 3.0),
            (-9.798, -1.733, 1.0),
            (8.395, -5.340, -1.0),
            (-2.476, 9.212, -3.0),
            (-3.992, -7.686, -5.0),
            (6.708, 2.450, -7.0),
            (-4.029, 1.663, -9.0)
        ];

        println!("{:?}", result);

    }

}
