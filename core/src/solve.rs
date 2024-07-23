use super::*;

/// A type that can 'solve' a graph to get its resulting value.
pub trait Solve {
    /// Solve the graph for a resulting value.
    fn solve<G: Graph>(&mut self, graph: &mut G) -> Result<Value, ()>;
}