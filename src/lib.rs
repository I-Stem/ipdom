extern crate xml_parser;

mod dom;
mod utils;

pub use xml_parser::{from_file, StrTendril, Handle, RawToken, parse_text};
pub use dom::{IpDom, Predicate, Node, QualName, AttributeTypes, NodeData};

pub type ParseResult = Result<xml_parser::Tokenizer<xml_parser::TreeBuilder>, String>;

pub fn parse_file(filepath: &str) -> Result<IpDom, &'static str>{
    let r = from_file(filepath);

    parse_fragment(r)
}

// parse xml string directly 
pub fn parse_xml(xml: String) -> Result<IpDom, &'static str>{
    let r = parse_text(xml);
   
    parse_fragment(r)
}

fn parse_fragment(r: ParseResult) -> Result<IpDom, &'static str> {
    if let Ok(r) = r {
        if let Some(x) = r.sink().output(){
            let built = IpDom::from_fragment(&x);

            return Ok(built);
        }
    }

    Err("Could not parse the text")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_xml_text() {
        let xml = String::from("<root><key name='SECTION – A' type='dict'> \n <item type='dict'> <question type='string'> Write some formula </question> </item> \n </key> </root>");
        let dom = parse_xml(xml).unwrap();

        assert_eq!(dom.len(), 3)
    }
}