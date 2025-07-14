mod pairs;

use crate::usfm::*;
use pairs::Unpack;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "usfm.pest"]
pub struct UsfmParser;

pub fn to_book(pairs: Pairs<Rule>) -> Book {
    Book {
        contents: pairs.map(to_book_contents).collect(),
    }
}

pub fn to_book_contents(pair: Pair<Rule>) -> BookContents {
    use BookContents as C;
    let rule = pair.as_rule();
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();

    match rule {
        Rule::id => C::Id {
            code: to_book_identifier(pairs.next_str()),
            text: pairs.next_str().to_string(),
        },
        Rule::usfm => C::Usfm(pairs.next_str().to_string()),
        Rule::ide => C::Encoding(to_book_encoding(pairs.next_str())),
        Rule::sts => C::Status(pairs.next_value()),
        Rule::c => C::Chapter(pairs.next_value()),
        Rule::ca => C::AltChapter(pairs.next_value()),
        Rule::p => C::Paragraph(Paragraph {
            style: to_paragraph_style(pairs.next_str()),
            contents: pairs.map(to_paragraph_contents),
        }),
        Rule::pn => C::Paragraph(Paragraph {
            style: to_numbered_paragraph_style(pairs.next_str(), pairs.next_value()),
            contents: pairs.map(to_paragraph_contents),
        }),
        Rule::q => C::Poetry(Poetry {
            style: to_poetry_style(pairs.next_str()),
            contents: pairs.map(to_paragraph_contents),
        }),
        Rule::qn => C::Poetry(Poetry {
            style: to_numbered_poetry_style(pairs.next_str(), pairs.next_value()),
            contents: pairs.map(to_paragraph_contents),
        }),
        Rule::e => C::Element(Element {
            ty: to_element_type(pairs.next_str()),
            contents: pairs.map(to_element_contents),
        }),
        Rule::en => C::Element(Element {
            ty: to_numbered_element_type(pairs.next_str(), pairs.next_value()),
            contents: pairs.map(to_element_contents),
        }),
        Rule::em => C::Empty(to_empty_type(pairs.next_str())),
        _ => panic!("Unexpected rule {:?} in to_book_contents", rule),
    }
}

pub fn to_paragraph_contents(pair: Pair<Rule>) -> ParagraphContents {
    use ParagraphContents as C;
    let rule = pair.as_rule();
    if rule == Rule::line {
        return C::Line(pair.as_str().to_string());
    }
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    match rule {
        Rule::v => C::Verse(pairs.next_value()),
        Rule::k => C::Character(Character {
            ty: to_character_type(pairs.next_str()),
            contents: pairs.map_if(false, &[Rule::attrib, Rule::value], to_character_contents),
            attributes: pairs.map_if(true, &[Rule::attrib, Rule::value], to_attribute),
        }),
        Rule::f => C::Footnote(Footnote {
            style: to_footnote_style(pairs.next_str()),
            caller: to_caller(pairs.next_char()),
            elements: pairs.map(to_footnote_element),
        }),
        Rule::x => C::CrossRef(CrossRef {
            style: to_cross_ref_style(pairs.next_str()),
            caller: to_caller(pairs.next_char()),
            elements: pairs.map(to_cross_ref_element),
        }),
        _ => panic!("Unexpected rule {:?} in to_paragraph_contents", rule),
    }
}

pub fn to_element_contents(pair: Pair<Rule>) -> ElementContents {
    use ElementContents as C;
    let rule = pair.as_rule();
    if rule == Rule::line {
        return C::Line(pair.as_str().to_string());
    }
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    match rule {
        Rule::k => C::Character(Character {
            ty: to_character_type(pairs.next_str()),
            contents: pairs.map_if(false, &[Rule::attrib, Rule::value], to_character_contents),
            attributes: pairs.map_if(true, &[Rule::attrib, Rule::value], to_attribute),
        }),
        Rule::f => C::Footnote(Footnote {
            style: to_footnote_style(pairs.next_str()),
            caller: to_caller(pairs.next_char()),
            elements: pairs.map(to_footnote_element),
        }),
        Rule::x => C::CrossRef(CrossRef {
            style: to_cross_ref_style(pairs.next_str()),
            caller: to_caller(pairs.next_char()),
            elements: pairs.map(to_cross_ref_element),
        }),
        _ => panic!("Unexpected rule {:?} in to_element_contents", rule),
    }
}

pub fn to_character_contents(pair: Pair<Rule>) -> CharacterContents {
    use CharacterContents as C;
    let rule = pair.as_rule();
    if rule == Rule::line {
        return C::Line(pair.as_str().to_string());
    }
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    match rule {
        Rule::k | Rule::nk => C::Character(Character {
            ty: to_character_type(pairs.next_str()),
            contents: pairs.map_if(false, &[Rule::attrib, Rule::value], to_character_contents),
            attributes: pairs.map_if(true, &[Rule::attrib, Rule::value], to_attribute),
        }),
        _ => panic!("Unexpected rule {:?} in to_character_contents", rule),
    }
}

pub fn to_footnote_element(pair: Pair<Rule>) -> FootnoteElement {
    use FootnoteElement as C;
    let rule = pair.as_rule();
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    if rule == Rule::reference {
        return C::Reference(NoteReference {
            chapter: pairs.next_value(),
            separator: pairs.next_char(),
            verse: pairs.next_value(),
        });
    }
    C::Element(NoteElement {
        style: to_footnote_element_style(pairs.next_str()),
        contents: pairs.map(to_character_contents),
    })
}

pub fn to_cross_ref_element(pair: Pair<Rule>) -> CrossRefElement {
    use CrossRefElement as C;
    let rule = pair.as_rule();
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    if rule == Rule::reference {
        return C::Reference(NoteReference {
            chapter: pairs.next_value(),
            separator: pairs.next_char(),
            verse: pairs.next_value(),
        });
    }
    C::Element(NoteElement {
        style: to_cross_ref_element_style(pairs.next_str()),
        contents: pairs.map(to_character_contents),
    })
}

pub fn to_attribute(pair: Pair<Rule>) -> (String, String) {
    if pair.as_rule() == Rule::attrib {
        let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
        (pairs.next_str().to_string(), pairs.next_str().to_string())
    } else {
        ("lemma".to_string(), pair.as_str().to_string())
    }
}

pub fn to_paragraph_style(s: &str) -> ParagraphStyle {
    use ParagraphStyle::*;
    match s {
        "p" => Normal,
        "m" => Margin,
        "po" => Opening,
        "pr" => Right,
        "cls" => Closure,
        "pmo" => EmbeddedOpening,
        "pm" => Embedded,
        "pmc" => EmbeddedClosing,
        "pmr" => EmbeddedRefrain,
        "mi" => MarginIndented,
        "nb" => Basic,
        "pc" => Centered,
        "lit" => LiturgicalNote,
        _ => panic!("Unknown paragraph style: {:?}", s),
    }
}

pub fn to_numbered_paragraph_style(s: &str, n: u8) -> ParagraphStyle {
    use ParagraphStyle::*;
    match s {
        "pi" => Indented(n),
        "ph" => HangingIndented(n),
        _ => panic!(
            "Unknown numbered paragraph style: {:?} with number {}",
            s, n
        ),
    }
}

pub fn to_poetry_style(s: &str) -> PoetryStyle {
    use PoetryStyle::*;
    match s {
        "qr" => Right,
        "qc" => Centered,
        "qa" => AcrosticHeading,
        "qd" => Descriptive,
        _ => panic!("Unknown poetry style: {:?}", s),
    }
}

pub fn to_numbered_poetry_style(s: &str, n: u8) -> PoetryStyle {
    use PoetryStyle::*;
    match s {
        "q" => Normal(n),
        "qm" => Embedded(n),
        _ => panic!("Unknown numbered poetry style: {:?} with number {}", s, n),
    }
}

pub fn to_element_type(s: &str) -> ElementType {
    use ElementType::*;
    match s {
        "rem" => Remark,
        "h" => Header,
        "ip" => Intro,
        "ipi" => IndentedIntro,
        "im" => MarginIntro,
        "imi" => MarginIndentedIntro,
        "ipq" => QuotedIntro,
        "imq" => MarginQuotedIntro,
        "ipr" => RightIntro,
        "ib" => BlankIntro,
        "iot" => OutlineIntro,
        "iex" => BridgeIntro,
        "ie" => EndIntro,
        "cl" => ChapterLabel,
        "cp" => ChapterPublishedLabel,
        "cd" => ChapterDescription,
        "mr" => MajorReference,
        "sr" => Reference,
        "r" => Parallel,
        "d" => Descriptive,
        "sp" => Speaker,
        _ => panic!("Unknown element type: {:?}", s),
    }
}

pub fn to_numbered_element_type(s: &str, n: u8) -> ElementType {
    use ElementType::*;
    match s {
        "toc" => Contents(n),
        "toca" => AltContents(n),
        "imt" => MajorIntro(n),
        "is" => SectionIntro(n),
        "iq" => PoetryIntro(n),
        "ili" => ListIntro(n),
        "io" => EntryIntro(n),
        "imte" => MajorTitleEndingIntro(n),
        "mt" => MajorTitle(n),
        "mte" => MajorTitleEnding(n),
        "ms" => MajorSection(n),
        "s" => Section(n),
        "sd" => Division(n),
        _ => panic!("Unknown numbered element type: {:?} with number {}", s, n),
    }
}

pub fn to_empty_type(s: &str) -> EmptyType {
    use EmptyType::*;
    match s {
        "b" => Blank,
        "pb" => PageBreak,
        _ => panic!("Unknown empty type: {:?}", s),
    }
}

pub fn to_character_type(s: &str) -> CharacterType {
    use CharacterType::*;
    match s {
        "ior" => IntroOutline,
        "iqt" => IntroQuote,
        "rq" => InlineQuote,
        "vp" => PublishedVerse,
        "qs" => Selah,
        "qac" => AcrosticLetter,
        "add" => Addition,
        "bk" => BookQuote,
        "dc" => DeuteroAddition,
        "k" => Keyword,
        "nd" => Deity,
        "ord" => Ordinal,
        "pn" => Proper,
        "png" => Geographic,
        "addpn" => ProperAddition,
        "qt" => QuotedText,
        "sig" => Signature,
        "sls" => SecondaryText,
        "tl" => Transliterated,
        "wj" => Jesus,
        "em" => Emphasis,
        "bd" => Bold,
        "it" => Italic,
        "bdit" => BoldItalic,
        "no" => Normal,
        "sc" => SmallCap,
        "sup" => Superscipt,
        "ndx" => Index,
        "rb" => Ruby,
        "pro" => Pronunciation,
        "w" => Word,
        "wg" => GreekWord,
        "wh" => HebrewWord,
        "wa" => AramaicWord,
        "jmp" => Link,
        _ => panic!("Unknown character type: {:?}", s),
    }
}

pub fn to_footnote_style(s: &str) -> FootnoteStyle {
    use FootnoteStyle::*;
    match s {
        "f" => Footnote,
        "fe" => Endnote,
        _ => panic!("Unknown footnote style: {:?}", s),
    }
}

pub fn to_cross_ref_style(s: &str) -> CrossRefStyle {
    use CrossRefStyle::*;
    match s {
        "x" => CrossRef,
        _ => panic!("Unknown cross-reference style: {:?}", s),
    }
}

pub fn to_footnote_element_style(s: &str) -> FootnoteElementStyle {
    use FootnoteElementStyle::*;
    match s {
        "fq" => TranslationQuote,
        "fqa" => AltTranslationQuote,
        "fk" => Keyword,
        "fl" => Label,
        "fw" => Witness,
        "fp" => Paragraph,
        "ft" => Text,
        "fdc" => DeuteroText,
        "fm" => ReferenceMark,
        _ => panic!("Unknown footnote element style: {:?}", s),
    }
}

pub fn to_cross_ref_element_style(s: &str) -> CrossRefElementStyle {
    use CrossRefElementStyle::*;
    match s {
        "xk" => Keyword,
        "xq" => Quote,
        "xt" => Target,
        "xta" => ExtraTarget,
        "xop" => Origin,
        "xot" => OldTarget,
        "xnt" => NewTarget,
        "xdc" => DeuteroTarget,
        "rq" => InlineQuote,
        _ => panic!("Unknown cross-reference element style: {:?}", s),
    }
}

pub fn to_caller(c: char) -> Caller {
    use Caller::*;
    match c {
        '+' => Auto,
        '-' => None,
        _ => Some(c),
    }
}

pub fn to_book_identifier(s: &str) -> BookIdentifier {
    use BookIdentifier::*;
    match s {
        "GEN" => Genesis,
        "EXO" => Exodus,
        "LEV" => Leviticus,
        "NUM" => Numbers,
        "DEU" => Deuteronomy,
        "JOS" => Joshua,
        "JDG" => Judges,
        "RUT" => Ruth,
        "1SA" => OneSamuel,
        "2SA" => TwoSamuel,
        "1KI" => OneKings,
        "2KI" => TwoKings,
        "1CH" => OneChronicles,
        "2CH" => TwoChronicles,
        "EZR" => Ezra,
        "NEH" => Nehemiah,
        "EST" => Esther,
        "JOB" => Job,
        "PSA" => Psalms,
        "PRO" => Proverbs,
        "ECC" => Ecclesiastes,
        "SNG" => SongOfSongs,
        "ISA" => Isaiah,
        "JER" => Jeremiah,
        "LAM" => Lamentations,
        "EZK" => Ezekiel,
        "DAN" => Daniel,
        "HOS" => Hosea,
        "JOL" => Joel,
        "AMO" => Amos,
        "OBA" => Obadiah,
        "JON" => Jonah,
        "MIC" => Micah,
        "NAM" => Nahum,
        "HAB" => Habakkuk,
        "ZEP" => Zephaniah,
        "HAG" => Haggai,
        "ZEC" => Zechariah,
        "MAL" => Malachi,
        "MAT" => Matthew,
        "MRK" => Mark,
        "LUK" => Luke,
        "JHN" => John,
        "ACT" => Acts,
        "ROM" => Romans,
        "1CO" => OneCorinthians,
        "2CO" => TwoCorinthians,
        "GAL" => Galatians,
        "EPH" => Ephesians,
        "PHP" => Philippians,
        "COL" => Colossians,
        "1TH" => OneThessalonians,
        "2TH" => TwoThessalonians,
        "1TI" => OneTimothy,
        "2TI" => TwoTimothy,
        "TIT" => Titus,
        "PHM" => Philemon,
        "HEB" => Hebrews,
        "JAS" => James,
        "1PE" => OnePeter,
        "2PE" => TwoPeter,
        "1JN" => OneJohn,
        "2JN" => TwoJohn,
        "3JN" => ThreeJohn,
        "JUD" => Jude,
        "REV" => Revelation,
        "TOB" => Tobit,
        "JDT" => Judith,
        "ESG" => EstherGreek,
        "WIS" => WisdomOfSolomon,
        "SIR" => Sirach,
        "BAR" => Baruch,
        "LJE" => LetterOfJeremiah,
        "S3Y" => SongOfThreeYoungMen,
        "SUS" => Susanna,
        "BEL" => BelAndTheDragon,
        "1MA" => OneMaccabees,
        "2MA" => TwoMaccabees,
        "3MA" => ThreeMaccabees,
        "4MA" => FourMaccabees,
        "1ES" => OneEsdras,
        "2ES" => TwoEsdras,
        "MAN" => PrayerOfManasseh,
        "PS2" => Psalm151,
        "ODA" => Odes,
        "PSS" => PsalmsOfSolomon,
        "EZA" => EzraApocalypse,
        "5EZ" => FiveEzra,
        "6EZ" => SixEzra,
        "DAG" => DanielGreek,
        "PS3" => Psalms152To155,
        "2BA" => TwoBaruch,
        "LBA" => LetterOfBaruch,
        "JUB" => Jubilees,
        "ENO" => Enoch,
        "1MQ" => OneMeqabyan,
        "2MQ" => TwoMeqabyan,
        "3MQ" => ThreeMeqabyan,
        "REP" => Reproof,
        "4BA" => FourBaruch,
        "LAO" => LetterToLaodiceans,
        "FRT" => FrontMatter,
        "BAK" => BackMatter,
        "OTH" => OtherMatter,
        "INT" => IntroductionMatter,
        "CNC" => Concordance,
        "GLO" => Glossary,
        "TDX" => TopicalIndex,
        "NDX" => NamesIndex,
        "XXA" => ExtraA,
        "XXB" => ExtraB,
        "XXC" => ExtraC,
        "XXD" => ExtraD,
        "XXE" => ExtraE,
        "XXF" => ExtraF,
        "XXG" => ExtraG,
        _ => panic!("Unknown book identifier: {:?}", s),
    }
}

fn to_book_encoding(s: &str) -> BookEncoding {
    use BookEncoding::*;
    match s {
        "CP-1252" => CP1252,
        "CP-1251" => CP1251,
        "UTF-8" => UTF8,
        "UTF-16" => UTF16,
        _ => panic!("Unknown encoding: {:?}", s),
    }
}
