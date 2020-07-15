use super::{Handle,RawToken};
mod interface;
use std::cell::RefCell;
pub use interface::{Node};
use super::pyo3::prelude::*;
use std::collections::HashMap;

///! index pointed dom
#[pyclass]
#[derive(Debug)]
pub struct IpDom {
    #[pyo3(get, set)]
    pub nodes: Vec<Node>
}

#[pymethods]
impl IpDom {
    /// Create a new instance
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    /// Get the total number of nodes in the dom already
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Return the node at that index, or None if none
    pub fn nth(&self, index: usize) -> Option<Node>{
        if index < self.len(){
            Some(self.nodes[index].clone())
        }else{
            None
        }
    }

    // // find a predicate 
    // pub fn find(&self, predicate: Box<dyn Predicate>, next: usize) -> Find {
    //     Find {
    //         dom: &self,
    //         predicate,
    //         next
    //     }
    // }
}


impl IpDom {
    /// Create an IpDom from an RcDOM tree
    pub fn from_fragment(fragment: &Handle) -> Self{
        let mut dom = IpDom::new();

        recur(fragment, &mut dom, None, None);


        return dom;

        /// iteratively traverse the tree inserting nodes in the document 
        /// 
        /// Also take care of setting appropriate parent and prev markers
        fn recur(
            handle: &Handle,
            dom: &mut IpDom,
            parent: Option<usize>,
            prev: Option<usize>
        ) -> Option<usize>{
            // if a node has children, recurse into the children, else, insert this node and return the index 
            let children = handle.children.borrow();
            let data = &handle.data;

            // create a new node, insert and recur into the children
            let node = create_node(data);
            let index = append(dom, parent, prev, node);

            if !children.is_empty(){
                // insert an attribute 
                let mut prev = None;

                for child in children.iter(){
                    prev = recur(&child, dom, Some(index), prev);
                }
                
            }
            None
        }

        /// Append nodes to the document
        /// 
        /// Adjust all index pointers in every inserted node
        fn append(
            dom: &mut IpDom,
            parent: Option<usize>,
            prev: Option<usize>,
            data: (String, HashMap<String, String>)
        ) -> usize {
            let index = dom.len();

            let node = Node {
                index,
                parent,
                prev,
                next: None,
                name: data.0,
                first_child: None,
                last_child: None,
                attributes: data.1
            };

            dom.nodes.push(node);

            // update the parent index 
            if let Some(parent) = parent {
                let parent = &mut dom.nodes[parent];

                if parent.first_child.is_none(){
                    parent.first_child = Some(index);
                }

                parent.last_child = Some(index);
            }

            // update the previous next index 
            if let Some(prev) = prev {
                dom.nodes[prev].next = Some(index);
            }

            

            index
        }



        /// Create a node from the given raw token 
        fn create_node(token: &RefCell<RawToken>) ->(String, HashMap<String, String>){
            let data = token.borrow();

            let name = &data.name;
            let attr = get_attributes(token);


            (name.0.to_string(), attr)
        }

        /// Extract an attribute frrom a raw token 
        fn get_attributes(token: &RefCell<RawToken>) -> HashMap<String, String>{
            let data = token.borrow();
            let mut map = HashMap::new();

            let name = &data.name.0;
            let value = &data.value.as_ref();
            let attrs = &data.attributes;

            if let Some(value) = value {
                map.insert(name.to_string(), value.to_string());
            }

            // add the attributes
            for (key, value) in attrs{
                map.insert(key.to_string(), value.to_string());
            }

            map
        }
    }
}