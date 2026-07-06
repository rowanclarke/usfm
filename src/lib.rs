pub mod parser;
pub mod usfm;

pub use usfm::*;

use parser::UsfmParser;
use parser::{Rule, to_book};
use pest::Parser;

pub fn parse(input: &str) -> Result<Book, String> {
    let parsed = UsfmParser::parse(Rule::book, input)
        .map_err(|e| {
            e.renamed_rules(|rule| match rule {
                Rule::ntext | Rule::text => "text".into(),
                Rule::k => "character style (\\style ...\\style*)".into(),
                Rule::kn => "numbered character style (\\style1 ...\\style1*)".into(),
                Rule::f => "footnote (\\f ...\\f*)".into(),
                Rule::x => "cross-reference (\\x ...\\x*)".into(),
                Rule::fig => "figure (\\fig ...\\fig*)".into(),
                Rule::milestone => "milestone (\\style\\*)".into(),
                Rule::cat => "category (\\cat ...\\cat*)".into(),
                Rule::optbreak => "optional break (//)".into(),
                Rule::k_style => "character style name".into(),
                Rule::kn_style => "numbered character style name".into(),
                Rule::mn_style => "numbered milestone style name".into(),
                Rule::ms_style => "milestone style name".into(),
                Rule::fe_style => "footnote element style (\\ft, \\fk, \\fq, ...)".into(),
                Rule::xe_style => "cross-reference element style (\\xt, \\xk, ...)".into(),
                Rule::f_style => "footnote style (\\f, \\fe, \\ef)".into(),
                Rule::x_style => "cross-reference style (\\x, \\ex)".into(),
                Rule::p_style => "paragraph style".into(),
                Rule::pn_style => "numbered paragraph style".into(),
                Rule::q_style => "poetry style".into(),
                Rule::qn_style => "numbered poetry style".into(),
                Rule::e_type => "element type".into(),
                Rule::en_type => "numbered element type".into(),
                Rule::em_type => "empty marker type (\\b, \\pb, \\ib)".into(),
                Rule::caller => "note caller (+, -, or character)".into(),
                Rule::num => "number".into(),
                Rule::verse_num => "verse number".into(),
                Rule::cell_prefix => "table cell type (\\th, \\tc, ...)".into(),
                _ => format!("{:?}", rule),
            })
            .to_string()
        })?;
    Ok(to_book(parsed))
}
