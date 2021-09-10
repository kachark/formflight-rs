
// NOTE: having the component is enough to flag as an agent
/// Flags an entity as an Agent
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Agent(pub bool);

/// Flags an entity as a Target
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Target(pub bool);

/// Flags an entity as an obstacle (non-targetable)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obstacle(pub bool);

