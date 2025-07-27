use rkyv::{Archive, Deserialize, Serialize};
use std::{cmp, fmt, hash};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub struct Book {
    pub contents: Vec<BookContents>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum BookContents {
    Id { code: BookIdentifier, text: String },
    Usfm(String),
    Encoding(BookEncoding),
    Status(u16),
    Chapter(u16),
    AltChapter(u16),
    Paragraph(Paragraph),
    Poetry(Poetry),
    Element(Element),
    Empty(EmptyType),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum ParagraphContents {
    Verse(u16),
    Line(String),
    Character(Character),
    Footnote(Footnote),
    CrossRef(CrossRef),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum ElementContents {
    Line(String),
    Character(Character),
    Footnote(Footnote),
    CrossRef(CrossRef),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum CharacterContents {
    Line(String),
    Character(Character),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum FootnoteElement {
    Reference(NoteReference),
    Element(NoteElement<FootnoteElementStyle>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum CrossRefElement {
    Reference(NoteReference),
    Element(NoteElement<CrossRefElementStyle>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub struct Paragraph {
    pub style: ParagraphStyle,
    pub contents: Vec<ParagraphContents>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub struct Poetry {
    pub style: PoetryStyle,
    pub contents: Vec<ParagraphContents>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub struct Element {
    pub ty: ElementType,
    pub contents: Vec<ElementContents>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(
    bounds(
        __C: rkyv::validation::ArchiveContext,
        __C::Error: rkyv::rancor::Source,
    )
))]
pub struct Character {
    pub ty: CharacterType,
    #[rkyv(omit_bounds)]
    pub contents: Vec<CharacterContents>,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub struct Footnote {
    pub style: FootnoteStyle,
    pub caller: Caller,
    pub elements: Vec<FootnoteElement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub struct CrossRef {
    pub style: CrossRefStyle,
    pub caller: Caller,
    pub elements: Vec<CrossRefElement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub struct NoteReference {
    pub chapter: u16,
    pub separator: char,
    pub verse: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub struct NoteElement<NoteStyle: Archive>
where
    <NoteStyle as Archive>::Archived: fmt::Debug + cmp::Eq + hash::Hash,
{
    pub style: NoteStyle,
    pub contents: Vec<CharacterContents>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum FootnoteElementStyle {
    Reference,
    TranslationQuote,
    AltTranslationQuote,
    Keyword,
    Label,
    Witness,
    Paragraph,
    Text,
    DeuteroText,
    ReferenceMark,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum CrossRefElementStyle {
    Reference,
    Keyword,
    Quote,
    Target,
    ExtraTarget,
    Origin,
    OldTarget,
    NewTarget,
    DeuteroTarget,
    InlineQuote,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum CharacterType {
    IntroOutline,
    IntroQuote,
    InlineQuote,

    PublishedVerse,

    Selah,
    AcrosticLetter,

    Addition,
    BookQuote,
    DeuteroAddition,
    Keyword,
    Deity,
    Ordinal,
    Proper,
    Geographic,
    ProperAddition,
    QuotedText,
    Signature,
    SecondaryText,
    Transliterated,
    Jesus,

    Emphasis,
    Bold,
    Italic,
    BoldItalic,
    Normal,
    SmallCap,
    Superscipt,

    Index,
    Ruby,
    Pronunciation,
    Word,
    GreekWord,
    HebrewWord,
    AramaicWord,
    Link,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum FootnoteStyle {
    Footnote,
    Endnote,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum CrossRefStyle {
    CrossRef,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum ParagraphStyle {
    Normal,
    Margin,
    Opening,
    Right,
    Closure,
    EmbeddedOpening,
    Embedded,
    EmbeddedClosing,
    EmbeddedRefrain,
    Indented(u8),
    MarginIndented,
    Basic,
    Centered,
    HangingIndented(u8),
    LiturgicalNote,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum PoetryStyle {
    Normal(u8),
    Right,
    Centered,
    AcrosticHeading,
    Embedded(u8),
    Descriptive,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum ElementType {
    Remark,
    Header,
    Contents(u8),
    AltContents(u8),

    MajorIntro(u8),
    SectionIntro(u8),
    Intro,
    IndentedIntro,
    MarginIntro,
    MarginIndentedIntro,
    QuotedIntro,
    MarginQuotedIntro,
    RightIntro,
    PoetryIntro(u8),
    BlankIntro,
    ListIntro(u8),
    OutlineIntro,
    EntryIntro(u8),
    BridgeIntro,
    MajorTitleEndingIntro(u8),
    EndIntro,

    ChapterLabel,
    ChapterPublishedLabel,
    ChapterDescription,

    MajorTitle(u8),
    MajorTitleEnding(u8),
    MajorSection(u8),
    MajorReference,
    Section(u8),
    Reference,
    Parallel,
    // Inline,
    Descriptive,
    Speaker,
    Division(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum EmptyType {
    Blank,
    PageBreak,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum Caller {
    Auto,
    None,
    Some(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum BookIdentifier {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    OneSamuel,
    TwoSamuel,
    OneKings,
    TwoKings,
    OneChronicles,
    TwoChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSongs,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    OneCorinthians,
    TwoCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    OneThessalonians,
    TwoThessalonians,
    OneTimothy,
    TwoTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    OnePeter,
    TwoPeter,
    OneJohn,
    TwoJohn,
    ThreeJohn,
    Jude,
    Revelation,

    Tobit,
    Judith,
    EstherGreek,
    WisdomOfSolomon,
    Sirach,
    Baruch,
    LetterOfJeremiah,
    SongOfThreeYoungMen,
    Susanna,
    BelAndTheDragon,
    OneMaccabees,
    TwoMaccabees,
    ThreeMaccabees,
    FourMaccabees,
    OneEsdras,
    TwoEsdras,
    PrayerOfManasseh,
    Psalm151,
    Odes,
    PsalmsOfSolomon,

    EzraApocalypse,
    FiveEzra,
    SixEzra,
    DanielGreek,
    Psalms152To155,
    TwoBaruch,
    LetterOfBaruch,
    Jubilees,
    Enoch,
    OneMeqabyan,
    TwoMeqabyan,
    ThreeMeqabyan,
    Reproof,
    FourBaruch,
    LetterToLaodiceans,

    FrontMatter,
    BackMatter,
    OtherMatter,
    IntroductionMatter,
    Concordance,
    Glossary,
    TopicalIndex,
    NamesIndex,
    ExtraA,
    ExtraB,
    ExtraC,
    ExtraD,
    ExtraE,
    ExtraF,
    ExtraG,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Hash, PartialEq, Eq, Debug))]
pub enum BookEncoding {
    CP1252,
    CP1251,
    UTF8,
    UTF16,
}
