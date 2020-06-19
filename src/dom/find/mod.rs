use super::{IpDom, Node};
mod predicate;

pub use predicate::Predicate;

///! Find iterator interface
pub struct Find<'a> {
    pub dom: &'a IpDom,
    pub predicate: Box<dyn Predicate>,
    pub next: usize,
}


impl<'a> Iterator for Find<'a>{
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item>{
        while self.next < self.dom.len(){
            let node = self.dom.nth(self.next).unwrap();

            self.next += 1;
            
            if self.predicate.macthes(&node){
                return Some(node);
            }
            
        }
        None
    }
}

impl<'a> DoubleEndedIterator for Find<'a>{
    fn next_back(&mut self) -> Option<Node<'a>>{
        while self.next > 0 && self.next < self.dom.len(){
            let node = self.dom.nth(self.next).unwrap();

            self.next -= 1;

            if self.predicate.macthes(&node){
                return Some(node);
            }
        }
        None
    }
}