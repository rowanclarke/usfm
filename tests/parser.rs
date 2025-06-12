use std::{fs::read_to_string, path::PathBuf};

use pest::Parser;
use rkyv_usfm::{
    parser::{Rule, UsfmParser, to_book},
    usfm::*,
};

static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn parse_web_genesis() {
    use BookContents::{Chapter, Paragraph};
    use Caller::Auto;
    use FootnoteElement::{Element, Reference};
    use FootnoteElementStyle::Text;
    use ParagraphContents::{Footnote, Line, Verse};
    use ParagraphStyle::Normal;

    let file = PathBuf::from(MANIFEST_DIR).join("usfm/02-GENeng-web.usfm");
    let input = read_to_string(file).unwrap();

    let pairs = UsfmParser::parse(Rule::book, &input).unwrap();
    assert_eq!(
        to_book(pairs).contents[9..11],
        vec![
            Chapter(1),
            Paragraph {
                style: Normal,
                contents: vec![
                    Verse(1),
                    Line("In the beginning, God".into()),
                    Footnote {
                        style: FootnoteStyle::Footnote,
                        caller: Auto,
                        elements: vec![
                            Reference {
                                chapter: 1,
                                separator: ':',
                                verse: 1
                            },
                            Element {
                                style: Text,
                                contents: vec![CharacterContents::Line(
                                    "The Hebrew word rendered “God” is “א\u{5b1}ל\u{5b9}ה\u{5b4}\u{591}ים”\n(Elohim).".into()
                                )]
                            }
                        ]
                    },
                    Line(" created the heavens and the earth.".into()),
                    Verse(2),
                    Line("The earth was formless and empty. Darkness was on the surface of the deep and God’s Spirit was hovering over the surface\nof the waters.".into())
                ]
            }
        ]
    );
}
