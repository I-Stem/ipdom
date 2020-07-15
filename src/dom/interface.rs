use super::{StrTendril, IpDom, QualName};
use std::collections::HashMap;
use super::utils::matches;
use std::fmt;
use std::cell::RefCell;
use crate::pyo3::prelude::*;

/// Node internal representation

#[pyclass]
#[derive(Debug, Clone)]
pub struct RawNode {
    pub index: usize,
    pub parent: Option<usize>,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub first_child: Option<usize>,
    pub last_child: Option<usize>,
    pub data: NodeData
}

/// Holds the data stored in a raw node 
#[pyclass]
#[derive(Debug, Clone)]
pub struct NodeData {
    name: QualName,
    attributes: HashMap<String, AttributeTypes>
}

impl NodeData {
    pub fn new(name: QualName) -> Self {
        Self {
            name,
            attributes: HashMap::new()
        }
    }
    /// Insert an attribute 
    pub fn insert(&mut self, attr: (String, AttributeTypes)){
        self.attributes.insert(attr.0, attr.1);
    }

    /// get the name of this node 
    pub fn name(&self) -> &QualName {
        &self.name
    }

    /// Return a reference to the attributes
    pub fn attrs(&self) -> &HashMap<String, AttributeTypes> {
        &self.attributes
    }
}


/// Attribute types for node data 
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeTypes {
    Bool(bool),
    Str(String),
    Int(u32),    
    Null
}



impl AttributeTypes {
    pub fn from_str(key: &StrTendril, value: &StrTendril, name: &StrTendril) -> Self {
         // special cases 
         // question numbers should be converted to int 
         if matches(name, "question_number"){
            //parse
            let v: String = value.into();
            let v = v.trim_matches('.');
            
            let v = v.parse::<u32>().unwrap_or(0);

            return Self::Int(v);
        }

        // marks 
        if matches(name, "marks"){
            let v = value.parse::<u32>().unwrap_or(0);

            return Self::Int(v);
        }

        // boolean types
        if matches(key, "bool"){
            // check type 
            let v = if matches(value, "True"){
                true
            }else{
                false
            };

            return Self::Bool(v);
        }

        // int type 
        if matches(key, "int"){
            // get the value 
            let v = value.parse::<u32>().unwrap_or(0);

            return Self::Int(v);

        }

        // string types
        if matches(key, "str"){
            // val 
            return Self::Str(value.to_string());
        }

       
        

        Self::Null
    }

    pub fn read(&self) -> String {
        match self {
            Self::Bool(b) => {
                let out = if *b {"true"} else {"false"};

                out.to_string()
            },
            Self::Str(s) => s.to_string(),
            Self::Int(i) => i.to_string(),
            _ => "Null".to_string() 
        }
    }
}

/// To navigate raw nodes with ease
#[derive(Clone)]
pub struct Node<'a>{
    index: usize,
    dom: &'a IpDom
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "Node at index {:?}",
            self.index()
        )
    } 
}


impl<'a> Node<'a>{
    /// create a new instance
    pub fn new(dom: &'a IpDom, index: usize) -> Option<Self>{
        if index < dom.nodes.len(){
            Some(Self {
                index,
                dom 
            })
        }else{
            None
        }
    }

    // get the raw node 
    pub fn raw(&self) -> &RawNode {
        &self.dom.nodes[self.index]
    }


    // index 
    pub fn index(&self) -> usize {
        self.index
    }

    // data 
    pub fn data(&self) -> &NodeData {
        &self.raw().data
    }
}