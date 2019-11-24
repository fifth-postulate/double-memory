//! Blossom algorithm for a maximal matching.
//!
//! The [blossom algorithm](https://en.wikipedia.org/wiki/Blossom_algorithm) is a polynomial time algorithm O(|E||V|^2) for finding a
//! [maximal matching](https://en.wikipedia.org/wiki/Matching_(graph_theory)).
#[deny(missing_docs)]
extern crate petgraph;

pub mod memory;

use petgraph::{graph::EdgeIndex, Graph, Undirected};
use std::collections::HashSet as Set;
use std::hash::Hash;

/// Determine a maximal matching for a graph.
pub fn maximal_matching<'a, N, E, I>(
    graph: &'a Graph<N, E, Undirected, I>
) -> Matching<I> where I: Eq + Hash {
    let mut matching = Matching::empty();
    loop {
        let path = augmentation_path(&graph, &matching);
        if let Some(path) = path {
            matching = matching.augment_with(path);
        } else {
            break;
        }
    }
    matching
}

/// Represents a matching for a certain graph.
pub struct Matching<'a, I> where I: Eq + Hash {
    edges: Set<&'a EdgeIndex<I>>,
}

impl<'a, I> Matching<'a, I> where I : Eq + Hash {
    /// return an empty matching
    pub fn empty() -> Self {
        Self { edges: Set::new() }
    }

    pub fn augment_with(self, path: Path<'a, I>) -> Self {
        let path_edges = Set::from(path.into());
        let m: Set<&EdgeIndex<I>> = self.edges.difference(&path_edges).cloned().collect();
        let p: Set<&EdgeIndex<I>> = path_edges.difference(&self.edges).cloned().collect();

        let edges: Set<&EdgeIndex<I>> = m.union(&p).cloned().collect();
        Self { edges }
    }
}

fn augmentation_path<'a, N, E, I>(graph: &'a Graph<N, E, Undirected, I>, matching: &Matching<'a, I>) -> Option<Path<'a, I>> where I: Eq + Hash {
    None
}

/// A path within a graph
pub struct Path<'a, I> where I: Eq + Hash {
    segments: Vec<&'a EdgeIndex<I>>,
}

impl<'a, I> IntoIterator for Path<'a, I> where I: Eq + Hash {
    type Item = &'a EdgeIndex<I>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter()
    }
}

impl<'a, I> Into<Set<&'a EdgeIndex<I>>> for Path<'a, I> where I: Eq + Hash {
    fn into(self) -> Set<&'a EdgeIndex<I>> {
        let mut edges = Set::new();
        for segment in self.segments {
            edges.insert(segment);
        }
        edges
    }

}

#[cfg(test)]
mod test {
    #[test]
    fn trivial_test() {
        assert!(true);
    }
}
