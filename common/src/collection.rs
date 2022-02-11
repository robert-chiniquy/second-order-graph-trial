use std::collections::HashMap;

pub use partitions::PartitionVec;
use partitions::*;

use super::P;

// Make the API of PartitionVec more ergonomic
pub struct Collection {
    items: PartitionVec<P>,
    items_to_index: HashMap<P, usize>,
}

impl Collection {
    pub fn new() -> Self {
        Self {
            items: partition_vec![],
            items_to_index: Default::default(),
        }
    }

    pub fn items(self) -> PartitionVec<P> {
        self.items
    }

    pub fn union(&mut self, p1: P, p2: P) {
        let (p1_index, p2_index) = self.indexes(p1, p2);
        self.items.union(p1_index, p2_index);
    }

    fn indexes(&mut self, p1: P, p2: P) -> (usize, usize) {
        (self.index(p1), self.index(p2))
    }

    fn index(&mut self, p: P) -> usize {
        if let std::collections::hash_map::Entry::Vacant(e) = self.items_to_index.entry(p) {
            let ind = self.items.len();
            self.items.insert(ind, p);
            e.insert(ind);
            ind
        } else {
            *self.items_to_index.get(&p).unwrap()
        }
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self::new()
    }
}
