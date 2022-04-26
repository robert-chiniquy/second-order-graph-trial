mod logic;

use common::{Collection, PartitionVec, P, R};

/// Take a list of edges P->R, return a disjoint set of P faceted by edges to R
pub fn compute(edges: Vec<(P, R)>) -> PartitionVec<P> {
    let samesets = logic::run(edges);
    let mut c = Collection::new();
    for (p1, p2) in samesets {
        c.union(p1, p2);
    }
    c.items()
}
