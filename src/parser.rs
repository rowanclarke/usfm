mod pairs;

use crate::usfm::*;
use pairs::Unpack;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "usfm.pest"]
pub struct UsfmParser;

pub fn parse(input: &str) -> Result<Book, String> {
    let parsed = UsfmParser::parse(Rule::book, input).map_err(|e| {
        e.renamed_rules(|rule| match rule {
            Rule::ntext | Rule::text => "text".into(),
            Rule::k => "character style (\\style ...\\style*)".into(),
            Rule::kn => "numbered character style (\\style1 ...\\style1*)".into(),
            Rule::f => "footnote (\\f ...\\f*)".into(),
            Rule::x => "cross-reference (\\x ...\\x*)".into(),
            Rule::fig => "figure (\\fig ...\\fig*)".into(),
            Rule::ms => "milestone (\\style\\*)".into(),
            Rule::mn => "numbered milestone (\\style\\*)".into(),
            Rule::cat => "category (\\cat ...\\cat*)".into(),
            Rule::br => "optional break (//)".into(),
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

fn to_book(pairs: Pairs<Rule>) -> Book {
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
            text: pairs.next_str_opt().map(str::to_string),
        },
        Rule::usfm_ver => C::Usfm(pairs.next_str().to_string()),
        Rule::ide => C::Encoding(to_book_encoding(pairs.next_str())),
        Rule::sts => C::Status(pairs.next_value()),
        Rule::c => C::Chapter(pairs.next_value()),
        Rule::ca => C::AltChapter(pairs.next_value()),
        Rule::p => C::Paragraph(to_paragraph(pairs)),
        Rule::pn => C::Paragraph(to_numbered_paragraph(pairs)),
        Rule::q => C::Poetry(to_poetry(pairs)),
        Rule::qn => C::Poetry(to_numbered_poetry(pairs)),
        Rule::e => C::Element(to_element(pairs)),
        Rule::en => C::Element(to_numbered_element(pairs)),
        Rule::em => C::Empty(to_empty_type(pairs.next_str())),
        Rule::tr => C::TableRow(to_table_row(pairs)),
        Rule::esb => C::Sidebar(to_sidebar(pairs)),
        Rule::periph => C::Peripheral(pairs.next_str().to_string()),
        _ => panic!("Unexpected rule {:?} in to_book_contents", rule),
    }
}

pub fn to_paragraph_contents(pair: Pair<Rule>) -> ParagraphContents {
    use ParagraphContents as C;
    let rule = pair.as_rule();
    if rule == Rule::ntext || rule == Rule::text {
        return C::Line(pair.as_str().to_string());
    }
    let pairs: Unpack<'_, Rule> = pair.into_inner().into();
    match rule {
        Rule::br => C::OptionalBreak,
        Rule::fig => C::Figure(to_figure(pairs)),
        Rule::ms => C::Milestone(to_milestone(pairs)),
        Rule::mn => C::Milestone(to_numbered_milestone(pairs)),
        Rule::cat => C::Category(to_category(pairs)),
        Rule::v => C::Verse(to_verse(pairs)),
        Rule::k => C::Character(to_character(pairs)),
        Rule::kn => C::Character(to_numbered_character(pairs)),
        Rule::f => C::Footnote(to_footnote(pairs)),
        Rule::x => C::CrossRef(to_cross_ref(pairs)),
        _ => panic!("Unexpected rule {:?} in to_paragraph_contents", rule),
    }
}

pub fn to_element_contents(pair: Pair<Rule>) -> ElementContents {
    use ElementContents as C;
    let rule = pair.as_rule();
    if rule == Rule::ntext || rule == Rule::text {
        return C::Line(pair.as_str().to_string());
    }
    let pairs: Unpack<'_, Rule> = pair.into_inner().into();
    match rule {
        Rule::br => C::OptionalBreak,
        Rule::fig => C::Figure(to_figure(pairs)),
        Rule::ms => C::Milestone(to_milestone(pairs)),
        Rule::mn => C::Milestone(to_numbered_milestone(pairs)),
        Rule::cat => C::Category(to_category(pairs)),
        Rule::k => C::Character(to_character(pairs)),
        Rule::kn => C::Character(to_numbered_character(pairs)),
        Rule::f => C::Footnote(to_footnote(pairs)),
        Rule::x => C::CrossRef(to_cross_ref(pairs)),
        _ => panic!("Unexpected rule {:?} in to_element_contents", rule),
    }
}

pub fn to_character_contents(pair: Pair<Rule>) -> CharacterContents {
    use CharacterContents as C;
    let rule = pair.as_rule();
    if rule == Rule::ntext {
        return C::Line(pair.as_str().to_string());
    }
    let pairs: Unpack<'_, Rule> = pair.into_inner().into();
    match rule {
        Rule::br => C::OptionalBreak,
        Rule::fig => C::Figure(to_figure(pairs)),
        Rule::ms => C::Milestone(to_milestone(pairs)),
        Rule::mn => C::Milestone(to_numbered_milestone(pairs)),
        Rule::k => C::Character(to_character(pairs)),
        Rule::kn => C::Character(to_numbered_character(pairs)),
        Rule::f => C::Footnote(to_footnote(pairs)),
        Rule::x => C::CrossRef(to_cross_ref(pairs)),
        _ => panic!("Unexpected rule {:?} in to_character_contents", rule),
    }
}

pub fn to_sidebar_contents(pair: Pair<Rule>) -> SidebarContents {
    use SidebarContents as C;
    let rule = pair.as_rule();
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    match rule {
        Rule::p => C::Paragraph(to_paragraph(pairs)),
        Rule::pn => C::Paragraph(to_numbered_paragraph(pairs)),
        Rule::q => C::Poetry(to_poetry(pairs)),
        Rule::qn => C::Poetry(to_numbered_poetry(pairs)),
        Rule::e => C::Element(to_element(pairs)),
        Rule::en => C::Element(to_numbered_element(pairs)),
        Rule::em => C::Empty(to_empty_type(pairs.next_str())),
        Rule::tr => C::TableRow(to_table_row(pairs)),
        Rule::cat => C::Category(to_category(pairs)),
        _ => panic!("Unexpected rule {:?} in to_sidebar_contents", rule),
    }
}

fn to_paragraph(mut pairs: Unpack<Rule>) -> Paragraph {
    Paragraph {
        style: to_paragraph_style(pairs.next_str()),
        contents: pairs.map(to_paragraph_contents),
    }
}

fn to_numbered_paragraph(mut pairs: Unpack<Rule>) -> Paragraph {
    Paragraph {
        style: to_numbered_paragraph_style(pairs.next_str(), pairs.next_value_or(Rule::num, 1)),
        contents: pairs.map(to_paragraph_contents),
    }
}

fn to_poetry(mut pairs: Unpack<Rule>) -> Poetry {
    Poetry {
        style: to_poetry_style(pairs.next_str()),
        contents: pairs.map(to_paragraph_contents),
    }
}

fn to_numbered_poetry(mut pairs: Unpack<Rule>) -> Poetry {
    Poetry {
        style: to_numbered_poetry_style(pairs.next_str(), pairs.next_value_or(Rule::num, 1)),
        contents: pairs.map(to_paragraph_contents),
    }
}

fn to_element(mut pairs: Unpack<Rule>) -> Element {
    Element {
        ty: to_element_type(pairs.next_str()),
        contents: pairs.map(to_element_contents),
    }
}

fn to_numbered_element(mut pairs: Unpack<Rule>) -> Element {
    Element {
        ty: to_numbered_element_type(pairs.next_str(), pairs.next_value_or(Rule::num, 1)),
        contents: pairs.map(to_element_contents),
    }
}

fn to_character(mut pairs: Unpack<Rule>) -> Character {
    Character {
        ty: to_character_type(pairs.next_str()),
        contents: pairs.map_if(
            false,
            &[Rule::attrib, Rule::value, Rule::default_value],
            to_character_contents,
        ),
        attributes: pairs.map_if(
            true,
            &[Rule::attrib, Rule::value, Rule::default_value],
            to_attribute,
        ),
    }
}

fn to_numbered_character(mut pairs: Unpack<Rule>) -> Character {
    let style = pairs.next_str();
    let num: u8 = pairs.next_value_or(Rule::num, 1);
    Character {
        ty: to_numbered_character_type(style, num),
        contents: pairs.map_if(
            false,
            &[Rule::attrib, Rule::value, Rule::default_value],
            to_character_contents,
        ),
        attributes: pairs.map_if(
            true,
            &[Rule::attrib, Rule::value, Rule::default_value],
            to_attribute,
        ),
    }
}

fn to_footnote(mut pairs: Unpack<Rule>) -> Footnote {
    Footnote {
        style: to_footnote_style(pairs.next_str()),
        caller: to_caller(pairs.next_char()),
        elements: pairs.map(to_footnote_element),
    }
}

fn to_cross_ref(mut pairs: Unpack<Rule>) -> CrossRef {
    CrossRef {
        style: to_cross_ref_style(pairs.next_str()),
        caller: to_caller(pairs.next_char()),
        elements: pairs.map(to_cross_ref_element),
    }
}

pub fn to_footnote_element(pair: Pair<Rule>) -> FootnoteElement {
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    FootnoteElement {
        style: to_footnote_element_style(pairs.next_str()),
        contents: pairs.map(to_character_contents),
    }
}

pub fn to_cross_ref_element(pair: Pair<Rule>) -> CrossRefElement {
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    CrossRefElement {
        style: to_cross_ref_element_style(pairs.next_str()),
        contents: pairs.map(to_character_contents),
    }
}

fn to_verse(mut pairs: Unpack<Rule>) -> String {
    pairs.next_str().to_string()
}

fn to_category(mut pairs: Unpack<Rule>) -> String {
    pairs.next_str().to_string()
}

pub fn to_attribute(pair: Pair<Rule>) -> (String, String) {
    if pair.as_rule() == Rule::attrib {
        let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
        (pairs.next_str().to_string(), pairs.next_str().to_string())
    } else {
        // Rule::value or Rule::default_value
        ("lemma".to_string(), pair.as_str().to_string())
    }
}

pub fn to_table_row(pairs: Unpack<Rule>) -> TableRow {
    let mut cells = Vec::new();
    for pair in pairs.0 {
        match pair.as_rule() {
            Rule::cell => cells.push(to_table_cell(pair)),
            _ => {
                if let Some(last) = cells.last_mut() {
                    last.contents.push(to_paragraph_contents(pair));
                }
            }
        }
    }
    TableRow { cells }
}

pub fn to_table_cell(pair: Pair<Rule>) -> TableCell {
    let mut pairs: Unpack<'_, Rule> = pair.into_inner().into();
    let prefix = to_cell_prefix(pairs.next_str());
    let column: u8 = pairs.next_value();
    let contents = pairs.map(to_paragraph_contents);
    TableCell {
        prefix,
        column,
        contents,
    }
}

pub fn to_sidebar(pairs: Unpack<Rule>) -> Sidebar {
    Sidebar {
        contents: pairs.map(to_sidebar_contents),
    }
}

pub fn to_figure(pairs: Unpack<Rule>) -> Figure {
    Figure {
        contents: pairs.map_if(
            false,
            &[Rule::attrib, Rule::value, Rule::default_value],
            to_character_contents,
        ),
        attributes: pairs.map_if(
            true,
            &[Rule::attrib, Rule::value, Rule::default_value],
            to_attribute,
        ),
    }
}

pub fn to_milestone(mut pairs: Unpack<Rule>) -> Milestone {
    Milestone {
        style: to_milestone_style(pairs.next_str(), pairs.next_str_opt()),
        attributes: pairs.map_if(
            true,
            &[Rule::attrib, Rule::value, Rule::default_value],
            to_attribute,
        ),
    }
}

pub fn to_numbered_milestone(mut pairs: Unpack<Rule>) -> Milestone {
    Milestone {
        style: to_numbered_milestone_style(
            pairs.next_str(),
            pairs.next_value(),
            pairs.next_str_opt(),
        ),
        attributes: pairs.map_if(
            true,
            &[Rule::attrib, Rule::value, Rule::default_value],
            to_attribute,
        ),
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
        "lh" => ListHeader,
        "lf" => ListFooter,
        "d" => Descriptive,
        _ => panic!("Unknown paragraph style: {:?}", s),
    }
}

pub fn to_numbered_paragraph_style(s: &str, n: u8) -> ParagraphStyle {
    use ParagraphStyle::*;
    match s {
        "pi" => Indented(n),
        "ph" => HangingIndented(n),
        "mi" => MarginIndentedNum(n),
        "lim" => EmbeddedListEntry(n),
        "li" => ListEntry(n),
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
        "ipc" => CenteredIntro,
        "ilit" => LiturgicalIntro,
        "iot" => OutlineIntro,
        "iex" => BridgeIntro,
        "ie" => EndIntro,
        "cl" => ChapterLabel,
        "cp" => ChapterPublishedLabel,
        "cd" => ChapterDescription,
        "mr" => MajorReference,
        "sr" => Reference,
        "r" => Parallel,
        "sp" => Speaker,
        "restore" => Restore,
        "ms" => MajorSection(1),
        "mt" => MajorTitle(1),
        "s" => Section(1),
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
        "ib" => IntroBlank,
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
        "wl" => ForeignWord,
        "jmp" => Link,
        "efm" => ExtFootnoteRef,
        "fv" => FootnoteVerse,
        "fm" => FootnoteRef,
        "litl" => ListTotal,
        "lik" => ListKey,
        "liv" => ListValue(0),
        "pl" => InlineSubheading,
        "ref" => ScriptureRef,
        "ta" => TextAlternative,
        "va" => AltVerse,
        "xt" => CrossRefTarget,
        _ => panic!("Unknown character type: {:?}", s),
    }
}

pub fn to_numbered_character_type(s: &str, n: u8) -> CharacterType {
    use CharacterType::*;
    match s {
        "liv" => ListValue(n),
        _ => panic!("Unknown numbered character type: {:?} with number {}", s, n),
    }
}

pub fn to_footnote_style(s: &str) -> FootnoteStyle {
    use FootnoteStyle::*;
    match s {
        "f" => Footnote,
        "fe" => Endnote,
        "ef" => ExtendedFootnote,
        "efe" => ExtendedEndnote,
        _ => panic!("Unknown footnote style: {:?}", s),
    }
}

pub fn to_cross_ref_style(s: &str) -> CrossRefStyle {
    use CrossRefStyle::*;
    match s {
        "x" => CrossRef,
        "ex" => ExtendedCrossRef,
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
        "fv" => Verse,
        "fr" => Reference,
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
        "xo" => OriginRef,
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

pub fn to_numbered_milestone_style(s: &str, n: u8, b: Option<&str>) -> MilestoneStyle {
    use MilestoneStyle::*;
    let bound = to_milestone_bound(b);
    match (s, b) {
        ("qt", _) => QuotedText(n, bound),
        _ => panic!(
            "Unknown numbered milestone style: {:?}",
            b.map(|b| format!("{}-{}", s, b))
                .unwrap_or(format!("{}", s))
        ),
    }
}

pub fn to_milestone_style(s: &str, b: Option<&str>) -> MilestoneStyle {
    use MilestoneStyle::*;
    let bound = to_milestone_bound(b);
    match (s, b) {
        ("qt", _) => QuotedText(1, bound),
        ("ts", _) => TextSection(bound),
        ("t", _) => Text(bound),
        ("wj", _) => WordsOfJesus(bound),
        ("vid", None) => VerseId,
        _ => panic!(
            "Unknown milestone style: {:?}",
            b.map(|b| format!("{}-{}", s, b))
                .unwrap_or(format!("{}", s))
        ),
    }
}

pub fn to_milestone_bound(s: Option<&str>) -> MilestoneBound {
    match s {
        Some("s") => MilestoneBound::Start,
        Some("e") => MilestoneBound::End,
        None => MilestoneBound::None,
        Some(s) => panic!("Unknown milestone direction: {:?}", s),
    }
}

pub fn to_cell_prefix(s: &str) -> CellPrefix {
    use CellPrefix::*;
    match s {
        "th" => Header,
        "thr" => HeaderRight,
        "thc" => HeaderCenter,
        "tc" => Content,
        "tcr" => ContentRight,
        "tcc" => ContentCenter,
        _ => panic!("Unknown cell prefix: {:?}", s),
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
