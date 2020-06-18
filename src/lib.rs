extern crate xml_parser;

mod dom;
mod utils;

pub use xml_parser::{from_file, StrTendril, Handle, RawToken};
pub use dom::IpDom;

pub fn parse_file(filepath: &str) -> Result<IpDom, &'static str>{
    let r = from_file(filepath);

    if let Ok(r) = r {
        if let Some(x) = r.sink().output(){
            let built = IpDom::from_fragment(&x);

            return Ok(built);
        }
    }

    Err("Could not parse the file")
}