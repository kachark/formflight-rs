
// https://www.toptal.com/game/video-game-physics-part-ii-collision-detection-for-solid-objects

use na::{Vector2, Vector3};

// TODO import polynomial/root finding crate
//


pub struct BoundingCircle {

    center_x: f32,
    center_y: f32,
    radius: f32

}

pub struct BoudingSphere {

    center_x: f32,
    center_y: f32,
    center_z: f32,
    radius: f32

}

pub fn circle_collision(circle_1: &BoundingCircle, circle_2: &BoundingCircle, dt: f32) -> bool {

    // TODO
    // need velocity term

    // let x1 = Vector2::new(circle_1.center_x, circle_1.center_y);
    // let x2 = Vector2::new(circle_2.center_x, circle_2.center_y);

    // let d = x1.metric_distance(&x2) - circle_1.radius - circle_2.radius;

    false

}


#[cfg(test)]

#[test]
fn test_circle_collision() {

    let circle_1 = BoundingCircle{center_x: 0.0, center_y: 0.0, radius: 5.0};
    let circle_2 = BoundingCircle{center_x: 10.0, center_y: 0.0, radius: 5.0};

    circle_collision(&circle_1, &circle_2, 0.1);


}

