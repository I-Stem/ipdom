use super::{IpDom, Node, AttributeTypes};
mod predicate;
pub use predicate::Predicate;

///! Find iterator interface
pub struct Find<'a> {
    pub dom: &'a IpDom,
    pub predicate: Box<dyn Predicate>,
    pub next: usize,
}

impl <'a> Find <'a>{
    pub fn extract(&mut self) -> Vec<String>{
        let mut v = Vec::new();

        while self.next < self.dom.len() {
            let node = self.dom.nth(self.next).unwrap();

            self.next += 1;
            
            if self.predicate.macthes(&node){
                let data = node.data().read().unwrap();

                //extract the attribute 
                let attr = data.attrs();
                if let Some(q) = attr.get("question"){
                    if let AttributeTypes::Str(s) = q {
                        v.push(s.clone());
                    }
                   
                }

            }
        }

        v
    }
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