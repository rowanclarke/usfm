extern crate usfm;
use rkyv::{deserialize, rancor::Error};
use std::{fs::read_to_string, path::PathBuf};
use usfm::*;

static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn assert_web_genesis(genesis: &Book) {
    use BookContents as A;
    use Caller::Auto;
    use FootnoteElementStyle::{Reference, Text};
    use ParagraphContents as B;
    use ParagraphStyle::Normal;
    assert_eq!(
        genesis.contents[9..11],
        vec![
            A::Chapter(1),
            A::Paragraph(Paragraph{
                style: Normal,
                contents: vec![
                    B::Verse("1".into()),
                    B::Line("In the beginning, God".into()),
                    B::Footnote(Footnote {
                        style: FootnoteStyle::Footnote,
                        caller: Auto,
                        elements: vec![
                            FootnoteElement {
                                style: Reference,
                                contents: vec![CharacterContents::Line("1:1  ".into())]
                            },
                            FootnoteElement {
                                style: Text,
                                contents: vec![CharacterContents::Line(
                                    "The Hebrew word rendered \u{201c}God\u{201d} is \u{201c}\u{05d0}\u{05b1}\u{05dc}\u{05b9}\u{05d4}\u{05b4}\u{0591}\u{05d9}\u{05dd}\u{201d}\n(Elohim).".into()
                                )]
                            }
                        ]
                    }),
                    B::Line(" created the heavens and the earth.".into()),
                    B::Verse("2".into()),
                    B::Line("The earth was formless and empty. Darkness was on the surface of the deep and God\u{2019}s Spirit was hovering over the surface\nof the waters.".into())
                ]
            })
        ]
    );
}

fn parse_web_genesis() -> Book {
    let file = PathBuf::from(MANIFEST_DIR).join("usfm/02-GENeng-web.usfm");
    let input = read_to_string(file).unwrap();
    parse(&input).unwrap()
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

    let book = parse(&input).unwrap();
    println!("{:?}", book.contents);
}

#[test]
fn parse_sample() {
    let file = PathBuf::from(MANIFEST_DIR).join("usfm/sample.usfm");
    let input = read_to_string(file).unwrap();
    let book = parse(&input).unwrap();
    println!("{:?}", book.contents);
}
