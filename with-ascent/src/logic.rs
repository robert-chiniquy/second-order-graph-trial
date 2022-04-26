#![allow(
    unused,
    clippy::field_reassign_with_default,
    clippy::clone_on_copy,
    clippy::collapsible_if,
    clippy::unused_unit
)]

use ascent::ascent;

use common::{P, R};

pub fn run(edges: Vec<(P, R)>) -> Vec<(P, P)> {
    let mut prog = AscentProgram::default();
    prog.edge = edges;
    prog.run();
    prog.same_set
}

ascent! {
  // input
  relation edge(P, R);

  relation in_p(P);
  relation different_sets(P,P);

  // output
  relation same_set(P,P);

  in_p(p) <-- edge(p, _);

  // Any P1, P2 which do not share any one edge go in different sets
  different_sets(p1, p2) <-- edge(p1, r), in_p(p2), if p1 != p2, !edge(p2, r);

  // Any P1, P2 which are not in different sets go in the same set
  same_set(p1, p2) <-- in_p(p1), in_p(p2), if p1 != p2, !different_sets(p1, p2);
}

#[test]
#[ignore]
fn f() {
    let facet_adjs = run(vec![(10, 20), (11, 20), (12, 21), (13, 22)]);

    // make test fail to see output
    assert_eq!(facet_adjs, Vec::default());
}
