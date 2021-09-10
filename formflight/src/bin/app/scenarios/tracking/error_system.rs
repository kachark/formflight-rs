
use legion::*;
use formflight::ecs::resources::TargetableSet;
use formflight::ecs::components::FullState;
use crate::scenarios::tracking::components::Agent;

#[system(for_each)]
pub fn print_error_state(
    _agent: &Agent,
    state1: &FullState,
    #[resource] targetable_set: &TargetableSet
)
{

    for (_id, state2) in targetable_set.0.iter() {
        if state1.data.len() != state2.data.len() {
            continue;
        } else {
            println!("ERROR STATE: {:?}", &state1.data  - &state2.data);
        }
    }

}


