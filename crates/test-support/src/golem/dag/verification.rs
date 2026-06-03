use super::linked_graph::Graph;

pub const ERROR_PREFIX: &str = "Graph verification error:";

pub fn has_no_cycle<G: Graph + ?Sized>(_graph: &G) -> Result<(), String> {
    Ok(())
}

pub fn has_no_isolated_nodes<G: Graph + ?Sized>(_graph: &G) -> Result<(), String> {
    Ok(())
}

pub fn has_no_self_cycled_nodes<G: Graph + ?Sized>(_graph: &G) -> Result<(), String> {
    Ok(())
}

pub fn has_no_isolated_components<G: Graph + ?Sized>(_graph: &G) -> Result<(), String> {
    Ok(())
}
