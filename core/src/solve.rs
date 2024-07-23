use super::*;

/// A type that can 'solve' a graph to get its resulting value.
pub trait Solver {
    /// Solve the graph for a resulting value.
    fn solve<G: Graph>(&mut self, graph: &mut G) -> Result<Value, ()>;
}

/// A solver that can operate concurrently, through the use of [`UnsafeGraph`].
pub trait ParSolver: Solver {
    /// Solve the graph for a resulting value.
    fn solve_par<G: UnsafeGraph>(&mut self, graph: &G) -> Result<Value, ()>;
}