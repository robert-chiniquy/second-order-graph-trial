pub mod logic;

use common::*;
use logic::*;

/// Take a list of edges P->R, return a disjoint set of P faceted by edges to R
pub fn compute(edges: Vec<(P, R)>) -> PartitionVec<P> {
    let samesets = logic::run(edges);
    let mut c = Collection::new();
    for SameSet(p1, p2) in samesets {
        c.union(p1, p2);
    }
    c.items()
}
