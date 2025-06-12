#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    pub contents: Vec<BookContents>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BookContents {
    Id {
        code: BookIdentifier,
        text: String,
    },
    Usfm(String),
    Encoding(BookEncoding),
    Status(u16),
    Chapter(u16),
    AltChapter(u16),
    Paragraph {
        style: ParagraphStyle,
        contents: Vec<ParagraphContents>,
    },
    Poetry {
        style: PoetryStyle,
        contents: Vec<ParagraphContents>,
    },
    Element {
        ty: ElementType,
        contents: Vec<ElementContents>,
    },
    Empty {
        ty: EmptyType,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParagraphContents {
    Verse(u16),
    Line(String),
    Character {
        ty: CharacterType,
        contents: Vec<CharacterContents>,
    },
    Footnote {
        style: FootnoteStyle,
        caller: Caller,
        elements: Vec<FootnoteElement>,
    },
    CrossRef {
        style: CrossRefStyle,
        caller: Caller,
        elements: Vec<CrossRefElement>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ElementContents {
    Line(String),
    Character {
        ty: CharacterType,
        contents: Vec<CharacterContents>,
    },
    Footnote {
        style: FootnoteStyle,
        caller: Caller,
        elements: Vec<FootnoteElement>,
    },
    CrossRef {
        style: CrossRefStyle,
        caller: Caller,
        elements: Vec<CrossRefElement>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum CharacterContents {
    Line(String),
    Character {
        ty: CharacterType,
        contents: Vec<CharacterContents>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum FootnoteElement {
    Reference {
        chapter: u16,
        separator: char,
        verse: u16,
    },
    Element {
        style: FootnoteElementStyle,
        contents: Vec<CharacterContents>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrossRefElement {
    Reference {
        chapter: u16,
        separator: char,
        verse: u16,
    },
    Element {
        style: CrossRefElementStyle,
        contents: Vec<CharacterContents>,
    },
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum FootnoteStyle {
    Footnote,
    Endnote,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrossRefStyle {
    CrossRef,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum PoetryStyle {
    Normal(u8),
    Right,
    Centered,
    AcrosticHeading,
    Embedded(u8),
    Descriptive,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum EmptyType {
    Blank,
    PageBreak,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Caller {
    Auto,
    None,
    Some(char),
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum BookEncoding {
    CP1252,
    CP1251,
    UTF8,
    UTF16,
}
