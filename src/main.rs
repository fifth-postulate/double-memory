extern crate matching;

use matching::memory::graph;

fn main() {
    for n in 2..11 {
        let g = graph(n);
        println!("n: {} v: {} e: {} c: {}", n, g.node_count(), g.edge_count(), n * (n-1) * (n-2)/2);
    }
}