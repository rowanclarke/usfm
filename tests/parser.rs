extern crate usfm;
use rkyv::{deserialize, rancor::Error};
use std::{fs::read_to_string, path::PathBuf};
use usfm::*;

static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn assert_web_genesis(genesis: &Book) {
    use BookContents as A;
    use Caller::Auto;
    use FootnoteElement as C;
    use FootnoteElementStyle::Text;
    use ParagraphContents as B;
    use ParagraphStyle::Normal;
    assert_eq!(
        genesis.contents[9..11],
        vec![
            A::Chapter(1),
            A::Paragraph(Paragraph{
                style: Normal,
                contents: vec![
                    B::Verse(1),
                    B::Line("In the beginning, God".into()),
                    B::Footnote(Footnote {
                        style: FootnoteStyle::Footnote,
                        caller: Auto,
                        elements: vec![
                            C::Reference(NoteReference {
                                chapter: 1,
                                separator: ':',
                                verse: 1
                            }),
                            C::Element(NoteElement {
                                style: Text,
                                contents: vec![CharacterContents::Line(
                                    "The Hebrew word rendered “God” is “א\u{5b1}ל\u{5b9}ה\u{5b4}\u{591}ים”\n(Elohim).".into()
                                )]
                            })
                        ]
                    }),
                    B::Line(" created the heavens and the earth.".into()),
                    B::Verse(2),
                    B::Line("The earth was formless and empty. Darkness was on the surface of the deep and God’s Spirit was hovering over the surface\nof the waters.".into())
                ]
            })
        ]
    );
}

fn parse_web_genesis() -> Book {
    let file = PathBuf::from(MANIFEST_DIR).join("usfm/02-GENeng-web.usfm");
    let input = read_to_string(file).unwrap();
    parse(&input)
}

#[test]
fn web_genesis() {
    let genesis = parse_web_genesis();
    assert_web_genesis(&genesis);
}

#[test]
fn rkyv_web_genesis() {
    let genesis = parse_web_genesis();
    let bytes = rkyv::to_bytes::<Error>(&genesis).unwrap();
    let archived = rkyv::access::<ArchivedBook, Error>(&bytes).unwrap();
    let genesis = &deserialize::<_, Error>(archived).unwrap();
    assert_web_genesis(genesis);
}

#[test]
fn parse_webpb_genesis() {
    let file = PathBuf::from(MANIFEST_DIR).join("usfm/02-GENengwebpb.usfm");
    let input = read_to_string(file).unwrap();

    let book = parse(&input);
    println!("{:?}", book.contents);
}
