# Double Sided Memory
A puzzle posed after learning about a game played with [_Kumbu_][kumbu]. Kumbu is

> an educational game with pictures on both sides. By playing, your memory and math skills will improve.  This game contains 4 game variants with different difficulties. It's fun for all ages. Kumbukumbu means memory in swahili.

## Origin
[@phedny][phedny] told me about a game. It is about memory but with pictures on both sides. The pictures are the numbers 0 through 9, and each combination occurs exactly once. Hence there are `10*9/2 = 45` tiles. Since this number is odd, at least one tile can't be matched.

We asked the question: **It is possible to have a group of more tiles left over?**

## Solution
After I have started working on implementing the solution angle below, @phedny gave an answer to the above question: **yes**.

### Reasoning
We can work towards leaving

```
(0, 1), (2, 3), (4,5)
```

We will do this in the following phases

1. Work towards leaving `(0, 1)`
2. Work towards leaving `(2, 3)`
3. Work towards leaving `(4, 5)`
4. Match remaining

```
(0, 1) (0, 2) (0, 3) (0, 4) (0, 5) (0, 6) (0, 7) (0, 8) (0, 9)
(1, 2) (1, 3) (1, 4) (1, 5) (1, 6) (1, 7) (1, 8) (1, 9)
(2, 3) (2, 4) (2, 5) (2, 6) (2, 7) (2, 8) (2, 9)
(3, 4) (3, 5) (3, 6) (3, 7) (3, 8) (3, 9)
(4, 5) (4, 6) (4, 7) (4, 8) (4, 9)
(5, 6) (5, 7) (5, 8) (5, 9)
(6, 7) (6, 8) (6, 9)
(7, 8) (7, 9)
(8, 9)
````````````

#### Phase `(0, 1)`
In order to leave `(0, 1)` we match `(0, i)` with `(1, i)`, where `i` runs from 2 to 9.

#### Phase `(2, 3)`
In order to leave both `(0, 1)` and `(2, 3)` we match `(2, j)` with `(3, j)`, where `j` runs from 4 to 9.

#### Phase `(4, 5)`
In order ro leave `(0,1)`, `(2, 3)` and `(4, 5)` we match `(4, k)` with `(5, k)`, where `k` runs from 6 to 9.

#### Match remaining
The remaining tiles can be matched in the following way

1. `(6, 7)` with `(6, 8)`
2. `(6, 9)` with `(7, 9)`
3. `(7, 8)` with `(8, 9)`

### Leaving 5
With some thought one can also find that 5 tiles can be left.

#### Phase `(6, 7)` & `(8, 9)`
In similar vain leaving `(6, 7)`, which also leaves `(8, 9)`, we match `(6, l)` with `(7, l)` where `l` runs from 8 to 9.

## Description
We will cast the question in terms of [graph theory][graph_theory]. The graph we will be analyzing will be composed of vertices. Each vertex corresponds with a tile and will be designated by a pair of numbers `(0,1), (0,2), (0,3), ..., (8,9)`.

There we be an edge between two vertices `(p,q)` and `(s,t)` if and only if either of the numbers on the tiles agree. I.e. `p` equals `s` or `t`, or `q` equals `s` or `t`. E.g. Vertex `(0,1)` is connected by an edges with with vertex `(0,2)`.

### Minimal criminal
A _minimal criminal_, i.e. an example of a play that leaves out more than one tile, leaves out three tiles. Since we can relabel the tiles so the minimal tiles that share no edges among them is

```
(0,1), (2,3), (4,5)
```

Removing these vertices from the graph and all incident edges leaves a graph. From this graph we need to determine if it has a perfect matching.

### Prefect Matching
We want to determine if a [perfect matching][matching] exists.

## Development
We will be using [Rust][rust] with the [petgraph][] crate. Before we can use it we need to implement a maximal matching algorithm. There is an [issue][blossem] that advocates for a perfect matching algorithm.

[kumbu]: https://www.rielekst.com/en/games/kumbu-kaartspel/
[phedny]: https://github.com/phedny
[graph_theory]: https://en.wikipedia.org/wiki/Graph_theory
[matching]: https://en.wikipedia.org/wiki/Matching_(graph_theory)
[rust]: https://www.rust-lang.org/
[petgraph]: https://crates.io/crates/petgraph
[blossem]: https://github.com/petgraph/petgraph/issues/296
