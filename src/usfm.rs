mod identifier;

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct Book {
    pub contents: Vec<BookContents>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub enum BookContents {
    Id {
        code: BookIdentifier,
        text: Option<String>,
    },
    Usfm(String),
    Encoding(BookEncoding),
    Status(u16),
    Chapter(u16),
    AltChapter(u16),
    Paragraph(Paragraph),
    Poetry(Poetry),
    Element(Element),
    Empty(EmptyType),
    TableRow(TableRow),
    Sidebar(Sidebar),
    Peripheral(String),
    Figure(Figure),
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub enum ParagraphContents {
    Verse(String),
    Line(String),
    Character(Character),
    Footnote(Footnote),
    CrossRef(CrossRef),
    Figure(Figure),
    Milestone(Milestone),
    Category(String),
    OptionalBreak,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub enum ElementContents {
    Line(String),
    Character(Character),
    Footnote(Footnote),
    CrossRef(CrossRef),
    Figure(Figure),
    Milestone(Milestone),
    Category(String),
    OptionalBreak,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub enum CharacterContents {
    Line(String),
    Character(Character),
    Footnote(Footnote),
    CrossRef(CrossRef),
    Figure(Figure),
    Milestone(Milestone),
    OptionalBreak,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct Paragraph {
    pub style: ParagraphStyle,
    pub contents: Vec<ParagraphContents>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct Poetry {
    pub style: PoetryStyle,
    pub contents: Vec<ParagraphContents>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct Element {
    pub ty: ElementType,
    pub contents: Vec<ElementContents>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
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

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct Footnote {
    pub style: FootnoteStyle,
    pub caller: Caller,
    pub elements: Vec<FootnoteElement>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct CrossRef {
    pub style: CrossRefStyle,
    pub caller: Caller,
    pub elements: Vec<CrossRefElement>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
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
pub struct FootnoteElement {
    pub style: FootnoteElementStyle,
    #[rkyv(omit_bounds)]
    pub contents: Vec<CharacterContents>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
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
pub struct CrossRefElement {
    pub style: CrossRefElementStyle,
    #[rkyv(omit_bounds)]
    pub contents: Vec<CharacterContents>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct TableCell {
    pub prefix: CellPrefix,
    pub column: u8,
    pub contents: Vec<ParagraphContents>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum CellPrefix {
    Header,
    HeaderRight,
    HeaderCenter,
    Content,
    ContentRight,
    ContentCenter,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
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
pub struct Figure {
    #[rkyv(omit_bounds)]
    pub contents: Vec<CharacterContents>,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct Milestone {
    pub style: MilestoneStyle,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum MilestoneStyle {
    QuotedText(u8, MilestoneBound),
    TextSection(MilestoneBound),
    Text(MilestoneBound),
    WordsOfJesus(MilestoneBound),
    VerseId,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum MilestoneBound {
    Start,
    End,
    None,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub struct Sidebar {
    pub contents: Vec<SidebarContents>,
}

#[derive(Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub enum SidebarContents {
    Paragraph(Paragraph),
    Poetry(Poetry),
    Element(Element),
    Empty(EmptyType),
    TableRow(TableRow),
    Category(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
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
    Verse,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum CrossRefElementStyle {
    Keyword,
    Quote,
    Target,
    ExtraTarget,
    Origin,
    OldTarget,
    NewTarget,
    DeuteroTarget,
    InlineQuote,
    OriginRef,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
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
    ForeignWord,
    Link,

    ExtFootnoteRef,
    FootnoteVerse,
    FootnoteRef,
    ListTotal,
    ListKey,
    ListValue(u8),
    InlineSubheading,
    ScriptureRef,
    TextAlternative,
    AltVerse,
    CrossRefTarget,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum FootnoteStyle {
    Footnote,
    Endnote,
    ExtendedFootnote,
    ExtendedEndnote,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum CrossRefStyle {
    CrossRef,
    ExtendedCrossRef,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
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
    MarginIndentedNum(u8),
    Basic,
    Centered,
    HangingIndented(u8),
    LiturgicalNote,
    ListHeader,
    ListFooter,
    Descriptive,
    ListEntry(u8),
    EmbeddedListEntry(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum PoetryStyle {
    Normal(u8),
    Right,
    Centered,
    AcrosticHeading,
    Embedded(u8),
    Descriptive,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
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
    CenteredIntro,
    LiturgicalIntro,
    PoetryIntro(u8),
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
    Speaker,
    Division(u8),
    Restore,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum EmptyType {
    Blank,
    PageBreak,
    IntroBlank,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum Caller {
    Auto,
    None,
    Some(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Archive, Serialize, Deserialize)]
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
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
#[rkyv(derive(Debug, PartialEq, Eq, Clone, Hash))]
pub enum BookEncoding {
    CP1252,
    CP1251,
    UTF8,
    UTF16,
}
