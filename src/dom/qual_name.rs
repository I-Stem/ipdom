use super::{StrTendril, utils::matches};

/// Qualified name of a node, used in the comparison finding
#[derive(Debug, PartialEq, Eq)]
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
        if matches(s, "SECTION_"){
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