use std::collections::{hash_map::Entry::*, HashMap};

use common::*;

/// Take a list of edges P->R, return a disjoint set of P faceted by edges to R
pub fn compute(edges: Vec<(P, R)>) -> PartitionVec<P> {
    // find the set of R connected to each P
    let mut p_to_rs: HashMap<P, Vec<R>> = Default::default();
    for (p, r) in edges {
        let entry = match p_to_rs.entry(p) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(Default::default()),
        };
        entry.push(r);
    }
    // get the set of P for each set of R
    let mut rs_to_ps: HashMap<Vec<R>, Vec<P>> = Default::default();
    for (p, rs) in p_to_rs {
        let entry = match rs_to_ps.entry(rs) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(Default::default()),
        };
        entry.push(p);
    }

    // TODO: Just not returning a PartitionVec might obviate this step, which would be faster
    let mut c = Collection::new();
    for (_, ps) in rs_to_ps {
        // logically we believe this will always have at least 1 item
        let p0 = ps[0];
        // intentionally union with itself
        for p in ps {
            c.union(p0, p);
        }
    }

    c.items()
}
