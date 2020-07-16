extern crate xml_parser;
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::exceptions;

mod dom;

pub use xml_parser::{from_file, StrTendril, Handle, RawToken, parse_text};
pub use dom::{IpDom};

pub type ParseResult = Result<xml_parser::Tokenizer<xml_parser::TreeBuilder>, String>;

pub fn parse_file(filepath: &str) -> PyResult<IpDom>{
    let r = from_file(filepath);

    parse_fragment(r)
}

// parse xml string directly 
pub fn parse_xml(xml: String) -> PyResult<IpDom>{
    let r = parse_text(xml);
   
    parse_fragment(r)
}

fn parse_fragment(r: ParseResult) -> PyResult<IpDom> {
    if let Ok(r) = r {
        if let Some(x) = r.sink().output(){
            let built = IpDom::from_fragment(&x);
            println!("{:#?}", built);
            
            return Ok(built);
        }
    }

    Err(exceptions::ValueError::py_err("Could not parse the text"))
}

// add binding for the generated python module
#[pymodule]
fn xmltodom(_py: Python, m: &PyModule) -> PyResult<()>{
    // the py arg represents that we are holding a GIL 
    #[pyfn(m, "parse_xml_string")]
    fn parse_xml_string(_py: Python, xml: String) -> PyResult<IpDom>{
        let out = parse_xml(xml);

        out 
    }


    Ok(())
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