use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::adjacency_list::AdjacencyList;

#[derive(Debug)]
pub struct Graph<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Display + Clone + Debug + Hash,
{
    nodes: RefCell<Vec<(T, T)>>,
}

#[derive(Debug)]
pub enum GraphOpError {
    TargetNotExists,
    ShouldBeDAG,
}

impl<T> Graph<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Display + Clone + Debug + Hash,
{
    pub const fn new() -> Self {
        Self {
            nodes: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&mut self, relation: (T, T)) {
        self.nodes.borrow_mut().push(relation);
    }

    pub fn remove(&mut self, relation: &(T, T)) -> Result<(), GraphOpError> {
        let mut target_idx_opt = None;
        let mut nodes_mut = self.nodes.borrow_mut();
        for (idx, rel) in nodes_mut.iter().enumerate() {
            if *rel == *relation {
                target_idx_opt.replace(idx);
                return Ok(());
            }
        }
        if let Some(target_idx) = target_idx_opt {
            nodes_mut.remove(target_idx);
            Ok(())
        } else {
            Err(GraphOpError::TargetNotExists)
        }
    }

    pub fn adjacency_list(&self) -> Result<AdjacencyList<T>, GraphOpError> {
        let mut adj_list = AdjacencyList::new();
        let nodes_ref = self.nodes.borrow_mut();

        let mut hs = HashSet::new();
        for (head, next) in nodes_ref.iter() {
            if hs.contains(next) {
                return Err(GraphOpError::ShouldBeDAG);
            }
            if hs.insert(head.clone()) {
                adj_list.add_head(head.clone()).unwrap();
            }
            adj_list.append_to_head(head.clone(), next.clone()).unwrap();
        }
        Ok(adj_list)
    }
}
