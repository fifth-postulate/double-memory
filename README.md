# Double Sided Memory
A puzzle posed after learning about a game.

## Origin
[@phedny][phedny] told me about a game. It is about memory but with pictures on both sides. The pictures are the numbers 0 through 9, and each combination occurs exactly once. Hence there are `10*9/2 = 45` tiles. Since this number is odd, at least one tile can't be match.

We asked the question: **It is possible to have a group of more tiles left over?**

## Description
We will cast the question in terms of [graph theory][graph_theory]. The graph we will be analyzing will be composed of vertices. Each vertex corresponds with a tile and will be designated by a pair of numbers `(0,1), (0,2), (0,3), ..., (8,9)`.

There we be an edge between two vertices `(p,q)` and `(s,t)` if and only if either of the numbers on the tiles agree. I.e. `p` equals `s` or `t`, or `q` equals `s` or `t`. E.g. Vertex `(0,1)` is connected by an edges with with vertex `(0,2).

### Minimal criminal
A _minimal criminal_, i.e. an example of a play that leaves out more than one tile, leaves out three tiles. Since we can relabel the tiles so the minimal tiles that share no edges among them is

```
(0, 1), (2,3), (4,5)
```

Removing these vertices from the graph and all incident edges leaves a graph. From this graph we need to determine if it has a perfect matching.

### Prefect Matching
We want to determine if a [perfect matching][matching] exists.

## Development
We will be using [Rust][rust] with the [petgraph][] crate. Before we can use it we need to implement a maximal matching algorithm. There is an [issue][blossem] that advocates for a perfect matching algorithm.

[phedny]: https://github.com/phedny
[graph_theory]: https://en.wikipedia.org/wiki/Graph_theory
[matching]: https://en.wikipedia.org/wiki/Matching_(graph_theory)
[rust]: https://www.rust-lang.org/
[petgraph]: https://crates.io/crates/petgraph
[blossem]: https://github.com/petgraph/petgraph/issues/296
