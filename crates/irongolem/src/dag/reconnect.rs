/// Defines how edges between a removed node's parents and children are treated.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ReconnectType {
    /// Do not reconnect predecessors.
    None,
    /// Reconnect predecessors only if the removed node had a single child.
    #[default]
    Single,
    /// Reconnect all predecessors to all successors.
    All,
}
