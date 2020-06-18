use super::Node;

/// Predicates are used to find different nodes in an ipDom
pub trait Predicate {
    fn macthes(&self, node: &Node) -> bool;
}