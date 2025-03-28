use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CGPEdges {
    edges: Vec<Vec<usize>>,
}


impl CGPEdges {
    pub fn new(nbr_nodes: usize) -> Self {
        let mut edges: Vec<Vec<usize>> = Vec::with_capacity(nbr_nodes);
        for _ in 0..nbr_nodes {
            edges.push(Vec::with_capacity(2));
        }

        Self {
            edges,
        }
    }

    /// Saves the edge; tail_id -> head_id
    pub fn add_edge(&mut self, head_id: usize, tail_id: usize) {
        self.edges[head_id].push(tail_id);
    }

    /// Removes one occurrence edge of prev_node_id from node_id.
    /// A second edge from tail_id -> head_id may exist, but the second occurrence will not be
    /// removed.
    pub fn remove_edge(&mut self, head_id: usize, tail_id: usize) {
        let index = self.edges[head_id]
            .iter()
            .position(|x| *x == tail_id)
            .unwrap();
        self.edges[head_id].swap_remove(index);
    }

    /// Returns true if tail_id -> head_id would lead to cycle
    pub fn leads_to_cycle(&self, head_id: usize, tail_id: usize) -> bool {
        let mut to_check: Vec<usize> = Vec::with_capacity(64);
        let mut checked: HashSet<usize, nohash_hasher::BuildNoHashHasher<usize>> = HashSet::default();

        to_check.extend(&self.edges[tail_id]);
        checked.extend(&self.edges[tail_id]);

        while let Some(checking) = to_check.pop() {
            if checking == head_id {
                return true;
            }

            for new_edge in &self.edges[checking] {
                if !checked.contains(new_edge) {
                    to_check.push(*new_edge);
                    checked.insert(*new_edge);
                }
            }
        }
        false
    }
}