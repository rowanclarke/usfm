use std::fs::read_to_string;
use usfm::{parse, format};

#[test]
fn test_roundtrip_sample() {
    let input = read_to_string("usfm/sample.usfm").expect("Failed to read sample.usfm");
    let book = parse(&input).expect("Failed to parse sample.usfm");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse formatted output");
    assert_eq!(book, reparsed, "Round-trip failed for sample.usfm");
}

#[test]
fn test_roundtrip_genesis() {
    let input = read_to_string("usfm/02-GENeng-web.usfm").expect("Failed to read genesis file");
    let book = parse(&input).expect("Failed to parse genesis file");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse formatted output");
    assert_eq!(book, reparsed, "Round-trip failed for genesis");
}

#[test]
fn test_roundtrip_genesis_pb() {
    let input = read_to_string("usfm/02-GENengwebpb.usfm").expect("Failed to read genesis pb file");
    let book = parse(&input).expect("Failed to parse genesis pb file");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse formatted output");
    assert_eq!(book, reparsed, "Round-trip failed for genesis pb");
}

#[test]
fn test_format_simple_paragraph() {
    let input = r#"\id GEN
\p Simple text
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_verse() {
    let input = r#"\id GEN
\c 1
\p
\v 1 In the beginning
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_bold_italic() {
    let input = r#"\id GEN
\p This is \bd bold\bd* and \it italic\it*
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_nested_character() {
    let input = r#"\id GEN
\p This is \bd bold with \it nested italic\it* inside\bd*
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_footnote() {
    let input = r#"\id GEN
\p Text\f + \fr 1:1 \ft Footnote text\f* continues
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_cross_ref() {
    let input = r#"\id GEN
\p Text\x + \xo 1:1 \xt Gen 2:1\x* continues
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_poetry() {
    let input = r#"\id GEN
\q1 First line of poetry
\q2 Second level indent
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_section_title() {
    let input = r#"\id GEN
\s1 Main Section
\p Paragraph text
\s2 Subsection
\p More text
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_chapter_verse() {
    let input = r#"\id GEN
\c 1
\p
\v 1 First verse
\v 2 Second verse
\v 3 Third verse
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_table() {
    let input = r#"\id GEN
\tr \th1 Header 1 \th2 Header 2
\tr \tc1 Cell 1 \tc2 Cell 2
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_empty_marker() {
    let input = r#"\id GEN
\p Text
\b
\p More text
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_encoding() {
    let input = r#"\id GEN
\ide UTF-8
\p Text
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_usfm_version() {
    let input = r#"\id GEN
\usfm 3.0
\p Text
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_introduction_elements() {
    let input = r#"\id GEN
\imt1 Introduction Title
\ip Introduction paragraph
\is1 Section heading
\ie
\p Body text
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_word_with_attributes() {
    let input = r#"\id GEN
\p \w sky|lemma="heaven" strong="H8064"\w*
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_word_with_default_value() {
    let input = r#"\id GEN
\p \w water|waters\w*
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_milestone() {
    let input = r#"\id GEN
\p text\qt-s\*more text\qt-e\*
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_optional_break() {
    let input = r#"\id GEN
\p First part // second part
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_sidebar() {
    let input = r#"\id GEN
\esb \cat History\cat*
\ms1 Sidebar Title
\p Sidebar content.
\esbe
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_caller_variants() {
    let input = r#"\id GEN
\p text\f + \ft auto caller\f* text\f - \ft no caller\f* text\f a \ft specific caller\f*
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);
    let reparsed = parse(&formatted).expect("Failed to reparse");
    assert_eq!(book, reparsed);
}

#[test]
fn test_format_output_is_valid_usfm() {
    let input = r#"\id GEN Test book
\usfm 3.0
\c 1
\p
\v 1 First verse with \bd bold\bd* text.
"#;
    let book = parse(input).expect("Failed to parse");
    let formatted = format(&book);

    // Verify the formatted output is valid USFM (can be parsed)
    let _reparsed = parse(&formatted).expect("Formatted output is not valid USFM");
}
