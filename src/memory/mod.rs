//! Create memory graphs
//!
//! Each vertex corresponds with a tile and will be designated by a pair of numbers (0,1), (0,2), (0,3), ..., (8,9).
//! There will be an edge between two vertices (p,q) and (s,t) if and only if either of the numbers on the tiles agree. I.e. p equals s or t, or q equals s or t. E.g. Vertex (0,1) is connected by an edges with with vertex (0,2).

use petgraph::{graph::Graph, Undirected};
use std::collections::BTreeMap as Dict;

/// Create a memory graph
pub fn graph(n: usize) -> Graph<(usize, usize), (), Undirected, u32> {
    let mut graph = Graph::with_capacity(n * (n-1)/2, 0);
    let mut indices = Dict::new();
    for first in 0..n {
        for second in (first + 1)..n {
            let label = (first, second);
            let index = graph.add_node(label);
            indices.insert(label, index);
        }
    }
    
    for origin in indices.keys() {
        for neighbor in indices
            .keys()
            .filter(|target| origin < target)
            .filter(|(p, q)| origin.0 == *p || origin.0 == *q || origin.1 == *p || origin.1 == *q)
        {
            let u_index = indices.get(origin).unwrap();
            let v_index = indices.get(neighbor).unwrap();

            graph.add_edge(*u_index, *v_index, ());
        }
    }

    graph
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn memory_10_graph_has_45_vertices() {
        let g = graph(10);

        assert_eq!(g.node_count(), 45);
    }

    #[test]
    fn memory_10_graph_has_360_edges() {
        let g = graph(10);

        assert_eq!(g.edge_count(), 360);
    }
}
