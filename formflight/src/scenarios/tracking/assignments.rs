
use std::error::Error;
use nalgebra::{DVector, DMatrix};
use rot::ot::emd::emd;
use rot::unbalanced::unbalanced_sinkhorn::sinkhorn_knopp_unbalanced;
use rot::utils::metrics::{dist, MetricType};

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
    let xs = DMatrix::<f64>::from_row_slice(nagents, dim_agents, xs_vec.as_slice());
    let xt = DMatrix::<f64>::from_row_slice(ntargets, dim_targets, xt_vec.as_slice());

    // Weights of discrete distribution masses representing agents/target states
    // For now: Uniform distribution
    let mut a = DVector::<f64>::from_vec(vec![1f64 / (nagents as f64); nagents]);
    let mut b = DVector::<f64>::from_vec(vec![1f64 / (ntargets as f64); ntargets]);

    // Get Euclidean distance cost between distributions of agent/target states
    let mut cost = dist(&xs, &xt, MetricType::SqEuclidean);

    // Normalize each cost row by it's maximum value
    for mut row in cost.row_iter_mut() {
        let max_ele = row.max();
        row.scale_mut(1f64 / max_ele);
    }

    // TODO: validate this summation
    // Check the weights of the source and target distributions
    // Get coupling matrix according to a given cost
    let gamma: DMatrix<f64>;
    let a_sum: f64 = a.iter().sum();
    let b_sum: f64 = b.iter().sum();
    if a_sum == b_sum {

        gamma = emd(&mut a, &mut b, &mut cost, None, None)?;

    } else {

        gamma = sinkhorn_knopp_unbalanced(&mut a, &mut b, &mut cost,
                                            0.1, 1.0, None, None)?;

    }

    // Convert coupling matrix to binary coupling matrix
    let mut binary = vec![vec![0; ntargets]; nagents];
    for (i, row) in gamma.row_iter().enumerate() {
        let threshold = row.max();
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

