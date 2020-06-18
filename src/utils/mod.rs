use regex::Regex;

//check if is an answer
pub fn matches<'a>(name: &'a str, pattern: &'static str) -> bool {
    let re = Regex::new(pattern).unwrap();

    return re.is_match(name);
}