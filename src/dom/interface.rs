use super::{ QualName};
use std::collections::HashMap;
use crate::pyo3::prelude::*;

/// Node internal representation

#[pyclass]
#[derive(Debug, Clone)]
pub struct Node {
    #[pyo3(get)]
    pub index: usize,
    #[pyo3(get)]
    pub parent: Option<usize>,
    #[pyo3(get)]
    pub prev: Option<usize>,
    #[pyo3(get)]
    pub next: Option<usize>,
    #[pyo3(get)]
    pub first_child: Option<usize>,
    #[pyo3(get)]
    pub last_child: Option<usize>,
    #[pyo3(get)]
    pub data: NodeData
}

/// Holds the data stored in a raw node 
#[pyclass]
#[derive(Debug, Clone)]
pub struct NodeData {
    name: QualName,
    attributes: HashMap<String, String>
}

impl NodeData {
    pub fn new(name: QualName) -> Self {
        Self {
            name,
            attributes: HashMap::new()
        }
    }
    /// Insert an attribute 
    pub fn insert(&mut self, attr: (String, String)){
        self.attributes.insert(attr.0, attr.1);
    }

    /// get the name of this node 
    pub fn name(&self) -> &QualName {
        &self.name
    }

    /// Return a reference to the attributes
    pub fn attrs(&self) -> &HashMap<String, String> {
        &self.attributes
    }
}

