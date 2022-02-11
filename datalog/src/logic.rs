use std::collections::HashSet;

use crepe::crepe;

use common::{P, R};

pub fn run(edges: Vec<(P, R)>) -> HashSet<SameSet> {
    let mut runtime = Crepe::new();
    runtime.extend(edges.into_iter().map(|(p, r)| Edge(p, r)));
    let (samesets,) = runtime.run();
    samesets
}

crepe! {
    @input
    struct Edge(P, R);

    struct DifferentSets(P, P);

    @output
    #[derive(Debug)]
    pub struct SameSet(pub P, pub P);

    // TODO: the use of _ here is probably dramatically inefficient

    // Any P1, P2 which do not share any one edge go in different sets
    DifferentSets(p1, p2) <- Edge(p1, r), Edge(p2, _), (p1 != p2), !Edge(p2, r);

    // Any P1, P2 which are not in different sets go in the same set
    SameSet(p1, p2) <- Edge(p1, _), Edge(p2, _), !DifferentSets(p1, p2);
}

#[test]
#[ignore]
fn f() {
    let mut runtime = Crepe::new();
    runtime.extend(&[Edge(10, 20), Edge(11, 20), Edge(12, 21), Edge(13, 22)]);

    let (facet_adjs,) = runtime.run();
    // make test fail to see output
    assert_eq!(facet_adjs, HashSet::default());
}
