# FormFlight-rs

![](https://github.com/kachark/formflight-rs/blob/main/images/trajectory_animation.gif)

This project aims to provide a first-look at the potential that optimal transport has in the areas of task assignment, resource allocation, and formation of dynamical systems.
Built using a highly parallelized dynamics simulator, individual systems are assigned terminal states to move to and are then modeled performing the needed actions to accomplish the task
given their dynamics and control laws.

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
```
