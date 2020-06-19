use super::{StrTendril, Handle,RawToken, utils};
mod interface;
mod qual_name;
use std::cell::RefCell;

pub use interface::{RawNode, Node, NodeData, AttributeTypes};
pub use qual_name::QualName;

mod find;
pub use find::{Find, Predicate};

///! index pointed dom
#[derive(Debug)]
pub struct IpDom {
    pub nodes: Vec<RawNode>
}

impl IpDom {
    /// Create a new instance
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
        Node::new(&self, index)
    }

    // find a predicate 
    pub fn find(&self, predicate: Box<Predicate>, next: usize) -> Find {
        Find {
            dom: &self,
            predicate,
            next
        }
    }
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

            if children.is_empty(){
                // insert an attribute 
                if let Some(attribute) = get_attribute(data){
                    return insert_attr(dom, parent, attribute);
                }
                
            }else{
                // create a new node, insert and recur into the children
                if let Some(node) = create_node(data){
                    let index = append(dom, parent, prev, node);

                    let mut prev = None;

                    for child in children.iter(){
                        prev = recur(&child, dom, Some(index), prev);
                    }
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
            data: NodeData
        ) -> usize {
            let index = dom.len();

            let node = RawNode {
                index,
                parent,
                prev,
                next: None,
                data: RefCell::new(data),
                first_child: None,
                last_child: None
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


        ///Insert an attribute to a given node in a given index 
        fn insert_attr(
            dom: &mut IpDom,
            index: Option<usize>,
            attribute: (StrTendril, AttributeTypes)
        ) -> Option<usize> {
            if let Some(index) = index {
                if let Some(node) = dom.nth(index){
                    // get the raw representation and insert
                    let raw = node.raw();
                    let mut data = raw.data.borrow_mut();
    
                    data.insert(attribute);
                }
            }
            

            index
        }

        /// Create a node from the given raw token 
        fn create_node(token: &RefCell<RawToken>) -> Option<NodeData>{
            let data = token.borrow();

            let name = &data.name;

            let qname = QualName::from_tendril(&name.0);

            return Some(NodeData::new(qname));
        }

        /// Extract an attribute frrom a raw token 
        fn get_attribute(token: &RefCell<RawToken>) -> Option<(StrTendril, AttributeTypes)>{
            let data = token.borrow();

            let name = &data.name.0;
            let value = &data.value.as_ref();
            let attrs = &data.attributes;

            if let Some(attr_type) = attrs.get(&StrTendril::from("type")){
                if let Some(value) = value {
                    let attr = AttributeTypes::from_str(attr_type, value, &name);

                    return Some((name.clone(), attr));
                }
                
            }


            None
        }
    }
}