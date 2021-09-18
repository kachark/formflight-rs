# FormFlight-rs

This project aims to provide a first-look at the potential that optimal transport has in the areas of task assignment, resource allocation, and formation of dynamical systems.

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
