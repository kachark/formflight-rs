# FormFlight-rs

This project aims to provide a first-look at the potential that optimal transport has in the areas of task assignment, resource allocation, and vehicle formation.

![](https://github.com/kachark/formflight-rs/blob/main/images/trajectory_animation.gif)

This image shows a distribution of dynamical systems (Agents) which has the task of moving into a target distribution (Targets). A centralized decision-maker
uses optimal transport to compute the assignment of individual Agents to Targets such that the group knows where to move to complete the task. In this example,
each Agent is driven with LQR controls.


## Dependencies
- [mads](https://github.com/kachark/mads)
- [rust-optimal-transport](https://github.com/kachark/rust-optimal-transport)

## Installation

### Cargo

Add the following to your Cargo.toml to install the necessary dependencies. NOTE: Since these dependencies are currently unpublished as crates, to update them to their latest commit use ```cargo update```.

```toml
[dependencies]
mads = { git = "https://github.com/kachark/mads", branch = "main" }
rust-optimal-transport = { git = "https://github.com/kachark/rust-optimal-transport", branch = "main" }
```

## Examples

### Run single simulation

From the root of the project directory:
```rust
cargo run
cd tools && python plot.py
```
