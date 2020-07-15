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
    pub name: String,
    #[pyo3(get)]
    pub attributes: HashMap<String, String>
}

