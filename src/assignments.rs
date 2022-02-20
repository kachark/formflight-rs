
use std::error::Error;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use ot::prelude::*;

// TODO: one single function for assignments that match for nagents = ntargets or not - actually
// check for the weights of the discrete distributions summing to the same value or not

pub fn ot_assignment(agent_states: &Vec<Vec<f32>>, target_states: &Vec<Vec<f32>>) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {

    let nagents = agent_states.len();
    let dim_agents = agent_states[0].len();

    let ntargets = target_states.len();
    let dim_targets = target_states[0].len();

    // Agent/target states in a single vector
    let mut xs_vec = Vec::new();
    for state in agent_states.iter() {
        for ele in state.iter() {
            xs_vec.push(*ele as f64);
        }
    }

    let mut xt_vec = Vec::<f64>::new();
    for state in target_states.iter() {
        for ele in state.iter() {
            xt_vec.push(*ele as f64);
        }
    }

    // Create matrices where each row is a state using vector slices
    // let xs = DMatrix::<f64>::from_row_slice(nagents, dim_agents, xs_vec.as_slice());
    // let xt = DMatrix::<f64>::from_row_slice(ntargets, dim_targets, xt_vec.as_slice());
    let source_samples = Array2::<f64>::from_shape_vec( (nagents, dim_agents), xs_vec )?;
    let target_samples = Array2::<f64>::from_shape_vec( (ntargets, dim_targets), xt_vec )?;

    // Weights of discrete distribution masses representing agents/target states
    // For now: Uniform distribution
    let mut source_weights = Array1::<f64>::from_vec(vec![1f64 / (nagents as f64); nagents]);
    let mut target_weights = Array1::<f64>::from_vec(vec![1f64 / (ntargets as f64); ntargets]);

    // Get Euclidean distance cost between distributions of agent/target states
    let mut cost = dist(&source_samples, &target_samples, SqEuclidean);

    cost = &cost / *cost.max().unwrap();

    // Check the weights of the source and target distributions
    // Get OT matrix according to a given cost
    let gamma = EarthMovers::new(&mut source_weights, &mut target_weights, &mut cost).solve()?;
    // let gamma = SinkhornKnopp::new(&source_weights, &target_weights, &cost, 1E-2).solve()?;

    // Convert coupling matrix to binary coupling matrix
    let mut binary = vec![vec![0; ntargets]; nagents];
    for (i, row) in gamma.axis_iter(Axis(0)).enumerate() {
        let threshold = row.max().unwrap();
        for (j, ele) in row.iter().enumerate() {
            if ele >= &threshold {
                binary[i][j] = 1;
            } else {
                binary[i][j] = 0;
            }
        }
    }

    Ok(binary)

}

