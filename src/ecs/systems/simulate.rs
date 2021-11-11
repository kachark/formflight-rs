#![allow(non_snake_case)]

// How Legion System macros work
// https://docs.rs/legion/0.4.0/legion/attr.system.html

use nalgebra::{DVector, DMatrix};
use legion::*;
use legion::storage::Component;
use mads::dynamics::statespace::StateSpaceRepresentation;
use mads::math::integrate::{solve_ivp, SolverOptions, IntegrateError, IntegratorType};
use mads::ecs::resources::*;
use mads::ecs::components::*;

use crate::ecs::resources::Assignment;
use crate::ecs::components::Agent;

// #[system(for_each)]
#[system(par_for_each)]
pub fn integrate_lqr_error_dynamics<T>(
    _agent: &Agent, // NOTE: test only evolving agents and NOT targets
    id: &SimID,
    state: &mut FullState,
    dynamics: &T,
    controller: &LQRComponent,
    #[resource] time: &SimulationTime,
    #[resource] sim_step: &EngineStep,
    #[resource] integrator: &Integrator,
    #[resource] step: &IntegratorStep,
    #[resource] assignment: &Assignment
) -> Result<(), IntegrateError>
where
    T: Component + StateSpaceRepresentation // Need to include Component trait from Legion
{

    // Define initial conditions
    let x0 = state.data.clone();
    let mut trajectory: Vec<DVector<f32>> = vec![x0.clone()];

    // Solve the LQR controller
    let (K, _P) = match controller.solve() {
        Ok((value1, value2)) => (value1, value2),
        Err(_) => (DMatrix::<f32>::zeros(1, 1), DMatrix::<f32>::zeros(1, 1)),
    };

    // Simulate
    let dt = sim_step.0;
    let step = step.0;
    let t0 = time.0;
    let tf = t0 + dt;
    let t_span = (t0, tf);
    let rtol = 1E-3;
    let _atol = 1E-6;

    // Compute error state for the controller
    let target_state = match assignment.map.get(&id.uuid) {

        Some(item) => {
            let stored = item.clone();
            match stored {

                Some(target_vector) => target_vector.clone(),
                None => DVector::<f32>::zeros(x0.len()),

            }
        },
        None => DVector::<f32>::zeros(x0.len())

    };

    let error_state = &x0 - &target_state;

    // Wrap dynamics/controls in appropriately defined closure - f(t, x)
    let f = |t: f32, x: &DVector<f32>| {
        let u = -&K * &error_state;
        dynamics.f(t, x, Some(&u))
    };

    // Integrate dynamics
    let opts = SolverOptions{ first_step: Some(step), rtol, ..SolverOptions::default() };
    let (_times, traj) = solve_ivp(f, t_span, x0, integrator.0, opts)?;

    // Update entity FullState component
    state.data = traj[traj.len()-1].clone();

    // Store result
    for state in traj {

        trajectory.push(state);

    }

    // DEBUG: show integrated dynamics
    // for x in trajectory {

    //     println!("{:?}", &x.data);

    // }

    Ok(())

}
