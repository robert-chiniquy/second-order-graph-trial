mod collection;

pub use collection::*;

// For a bipartite graph of {P, R},
// Partition P into sets (facets) such that every element in
// partition is adjacent to exactly the same nodes in R
pub type P = u32;
pub type R = u32;
