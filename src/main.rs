extern crate matching;

use matching::memory::graph;

fn main() {
    for n in 1..11 {
        let g = graph(n);
        println!("n: {} v: {} e: {}", n, g.node_count(), g.edge_count());
}
    }

