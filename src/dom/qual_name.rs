use super::{StrTendril, utils::matches};


/// Qualified name of a node, used in the comparison finding
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QualName {
    Document,
    Question,
    QuestionList,
    Section,
    Page,
    PageNumber,
    Metadata,
    Element
}


impl QualName {
    pub fn read(&self) -> String {
        match self {
            Self::Document => "document".to_string(),
            Self::Question => "question".to_string(),
            Self::QuestionList => "question_list".to_string(),
            Self::Section => "section".to_string(),
            Self::Page => "page".to_string(),
            Self::PageNumber => "page_number".to_string(),
            Self::Metadata => "meta_data".to_string(),
            _ =>  "element".to_string()
            
        }
    }
    pub fn from_tendril(s: &StrTendril) -> Self {
        // Document tag 
        if matches(s, "root"){
            return Self::Document;
        }

        // Question tag (item)
        if matches(s, "item"){
            return Self::Question;
        }

        // Question list 
        if matches(s, "question_list"){
            return Self::QuestionList;
        }

        // Section tag 
        if matches(s, "SECTION_") | matches(s, "key"){
            return Self::Section;
        }

        // Metadata
        if matches(s, "meta_data"){
            return Self::Metadata;
        }

        // Page break 
        if matches(s, r"^page_\d{1}$"){
            return Self::Page;
        }

        Self::Element 
    }
}