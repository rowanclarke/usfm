use std::str::FromStr;

use crate::usfm::*;
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
    use BookContents::*;
    let rule = pair.as_rule();
    let mut pairs = pair.into_inner();

    match rule {
        Rule::id => Id {
            code: to_book_identifier(pairs.next().unwrap().as_str()),
            text: pairs.next().unwrap().to_string(),
        },
        Rule::usfm => Usfm(pairs.next().unwrap().to_string()),
        Rule::encoding => Encoding(to_book_encoding(pairs.next().unwrap().as_str())),
        Rule::sts => Status(to_direct(pairs.next().unwrap())),
        Rule::c => Chapter(to_direct(pairs.next().unwrap())),
        Rule::ca => AltChapter(to_direct(pairs.next().unwrap())),
        Rule::p => Paragraph {
            style: to_paragraph_style(pairs.next().unwrap().as_str()),
            contents: pairs.map(to_paragraph_contents).collect(),
        },
        Rule::pn => Paragraph {
            style: to_numbered_paragraph_style(
                pairs.next().unwrap().as_str(),
                to_direct(pairs.next().unwrap()),
            ),
            contents: pairs.map(to_paragraph_contents).collect(),
        },
        Rule::q => Poetry {
            style: to_poetry_style(pairs.next().unwrap().as_str()),
            contents: pairs.map(to_paragraph_contents).collect(),
        },

        Rule::qn => Poetry {
            style: to_numbered_poetry_style(
                pairs.next().unwrap().as_str(),
                to_direct(pairs.next().unwrap()),
            ),
            contents: pairs.map(to_paragraph_contents).collect(),
        },
        Rule::e => Element {
            ty: to_element_type(pairs.next().unwrap().as_str()),
            contents: pairs.map(to_element_contents).collect(),
        },
        _ => unreachable!(),
    }
}

pub fn to_direct<T: FromStr>(pair: Pair<Rule>) -> T {
    T::from_str(pair.as_str()).unwrap_or_else(|_| panic!())
}

pub fn to_paragraph_contents(pair: Pair<Rule>) -> ParagraphContents {
    use ParagraphContents::*;
    let rule = pair.as_rule();
    if rule == Rule::line {
        return Line(pair.to_string());
    }
    let mut pairs = pair.into_inner();
    let mut next = || pairs.next().unwrap();
    let mut next_str = || next().as_str();
    match rule {
        Rule::v => Verse(to_direct(next())),
        Rule::k => Character {
            ty: to_character_type(next_str()),
            contents: pairs.map(to_character_contents).collect(),
        },
        Rule::f => Footnote {
            style: to_footnote_style(next_str()),
            caller: to_caller(next_str()),
            elements: pairs.map(to_footnote_element).collect(),
        },
        Rule::x => CrossRef {
            style: to_cross_ref_style(next_str()),
            caller: to_caller(next_str()),
            elements: pairs.map(to_cross_ref_element).collect(),
        },
        _ => unreachable!(),
    }
}

pub fn to_element_contents(pair: Pair<Rule>) -> ElementContents {
    use ElementContents::*;
    let rule = pair.as_rule();
    if rule == Rule::line {
        return Line(pair.to_string());
    }
    let mut pairs = pair.into_inner();
    let mut next = || pairs.next().unwrap();
    let mut next_str = || next().as_str();
    match rule {
        Rule::k => Character {
            ty: to_character_type(next_str()),
            contents: pairs.map(to_character_contents).collect(),
        },
        Rule::f => Footnote {
            style: to_footnote_style(next_str()),
            caller: to_caller(next_str()),
            elements: pairs.map(to_footnote_element).collect(),
        },
        Rule::x => CrossRef {
            style: to_cross_ref_style(next_str()),
            caller: to_caller(next_str()),
            elements: pairs.map(to_cross_ref_element).collect(),
        },
        _ => unreachable!(),
    }
}

pub fn to_character_contents(pair: Pair<Rule>) -> CharacterContents {
    use CharacterContents::*;
    let rule = pair.as_rule();
    if rule == Rule::line {
        return Line(pair.to_string());
    }
    let mut pairs = pair.into_inner();
    let mut next = || pairs.next().unwrap();
    let mut next_str = || next().as_str();
    match rule {
        Rule::k => Character {
            ty: to_character_type(next_str()),
            contents: pairs.map(to_character_contents).collect(),
        },
        _ => unreachable!(),
    }
}

pub fn to_footnote_element(pair: Pair<Rule>) -> FootnoteElement {
    let mut pairs = pair.into_inner();
    let mut next = || pairs.next().unwrap();
    let mut next_str = || next().as_str();
    FootnoteElement {
        style: to_footnote_element_style(next_str()),
        contents: pairs.map(to_character_contents).collect(),
    }
}

pub fn to_cross_ref_element(pair: Pair<Rule>) -> CrossRefElement {
    let mut pairs = pair.into_inner();
    let mut next = || pairs.next().unwrap();
    let mut next_str = || next().as_str();
    CrossRefElement {
        style: to_cross_ref_element_style(next_str()),
        contents: pairs.map(to_character_contents).collect(),
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
        _ => unreachable!(),
    }
}

pub fn to_numbered_paragraph_style(s: &str, n: u8) -> ParagraphStyle {
    use ParagraphStyle::*;
    match s {
        "pi" => Indented(n),
        "ph" => HangingIndented(n),
        _ => unreachable!(),
    }
}

pub fn to_poetry_style(s: &str) -> PoetryStyle {
    use PoetryStyle::*;
    match s {
        "qr" => Right,
        "qc" => Centered,
        "qa" => AcrosticHeading,
        "qd" => Descriptive,
        _ => unreachable!(),
    }
}

pub fn to_numbered_poetry_style(s: &str, n: u8) -> PoetryStyle {
    use PoetryStyle::*;
    match s {
        "q" => Normal(n),
        "qm" => Embedded(n),
        _ => unreachable!(),
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
        _ => unreachable!(),
    }
}

pub fn to_character_type(s: &str) -> CharacterType {
    use CharacterType::*;
    match s {
        "ior" => (),
        "iqt" => (),
        "rq" => (),
        "vp" => (),
        "qs" => (),
        "qac" => (),
        "add" => (),
        "bk" => (),
        "dc" => (),
        "k" => (),
        "nd" => (),
        "ord" => (),
        "pn" => (),
        "png" => (),
        "addpn" => (),
        "qt" => (),
        "sig" => (),
        "sls" => (),
        "tl" => (),
        "wj" => (),
        "em" => (),
        "bd" => (),
        "it" => (),
        "bdit" => (),
        "no" => (),
        "sc" => (),
        "sup" => (),
        "ndx" => (),
        "rb" => (),
        "pro" => (),
        "w" => (),
        "wg" => (),
        "wh" => (),
        "wa" => (),
        "jmp" => (),
        _ => unreachable!(),
    }
    todo!()
}

pub fn to_footnote_style(s: &str) -> FootnoteStyle {
    use FootnoteStyle::*;
    match s {
        "f" => Footnote,
        "fe" => Endnote,
        _ => unreachable!(),
    }
}

pub fn to_cross_ref_style(s: &str) -> CrossRefStyle {
    use CrossRefStyle::*;
    match s {
        "x" => CrossRef,
        _ => unreachable!(),
    }
}

pub fn to_footnote_element_style(s: &str) -> FootnoteElementStyle {
    use FootnoteElementStyle::*;
    match s {
        "fq" => (),
        "fqa" => (),
        "fk" => (),
        "fl" => (),
        "fw" => (),
        "fp" => (),
        "ft" => (),
        "fdc" => (),
        "fm" => (),
        _ => unreachable!(),
    }
    todo!()
}

pub fn to_cross_ref_element_style(s: &str) -> CrossRefElementStyle {
    use CrossRefElementStyle::*;
    match s {
        "xk" => (),
        "xq" => (),
        "xt" => (),
        "xta" => (),
        "xop" => (),
        "xot" => (),
        "xnt" => (),
        "xdc" => (),
        "rq" => (),
        _ => unreachable!(),
    }
    todo!()
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
        _ => unreachable!(),
    }
}

pub fn to_caller(s: &str) -> Caller {
    use Caller::*;
    match s {
        "+" => Auto,
        "-" => None,
        _ => Some(s.chars().next().unwrap()),
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
        _ => unreachable!(),
    }
}

fn to_book_encoding(s: &str) -> BookEncoding {
    use BookEncoding::*;
    match s {
        "CP-1252" => CP1252,
        "CP-1251" => CP1251,
        "UTF-8" => UTF8,
        "UTF-16" => UTF16,
        _ => unreachable!(),
    }
}
