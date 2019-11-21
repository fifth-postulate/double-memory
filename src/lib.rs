//! Blossom algorithm for a maximal matching.
//! 
//! The [blossom algorithm](https://en.wikipedia.org/wiki/Blossom_algorithm) is a polynomial time algorithm O(|E||V|^2) for finding a 
//! [maximal matching](https://en.wikipedia.org/wiki/Matching_(graph_theory)).
#[deny(missing_docs)]

extern crate petgraph;

pub mod memory;

use std::collections::HashSet as Set;
use petgraph::{Graph, Undirected, graph::EdgeIndex};

/// Determine a maximal matching for a graph.
pub fn maximal_matching<N, E, Ix>(graph: Graph<N, E, Undirected, Ix>) -> Option<Set<EdgeIndex<Ix>>> {
    None
}

#[cfg(test)]
mod test {
    #[test]
    fn trivial_test() {
        assert!(true);
    }
}
