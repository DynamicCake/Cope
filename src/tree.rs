use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
pub struct TreeNode<T: PartialEq> {
    pub data: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T: PartialEq> TreeNode<T>{

    fn new(data: T, children: Vec<TreeNode<T>>) -> Self {
        Self { data, children }
    }

    pub fn flatten(&self) -> Result<Vec<&TreeNode<T>>, CyclicError> {
        let mut all_nodes = Vec::new();
        match self.search_node(&mut all_nodes) {
            Ok(_) => {} 
            Err(err) => {
                return Err(err)
            }
        }

        Ok(all_nodes)
    }

    fn search_node<'a>(&'a self, all_nodes: &mut Vec<&'a TreeNode<T>>) -> Result<(), CyclicError> {

        if all_nodes.contains(&self) {
            return Err(CyclicError::new());
        }

        all_nodes.push(&self);

        if !self.children.is_empty() {
            for child in &self.children {
                let res = child.search_node(all_nodes);
                if res.is_err() {
                    return res;
                }
            }
        }

        return Ok(());
    }
}


#[derive(Debug)]
pub struct CyclicError;

impl Display for CyclicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cyclic reference detected")
    }
}

impl Error for CyclicError {}

impl CyclicError {
    fn new() -> Self {
        Self {}
    }
}