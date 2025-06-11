struct Book {
    contents: Vec<BookContents>,
}

enum BookContents {
    Id(),
    Usfm(),
    Encoding(),
    Status(),
    Chapter(),
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
}

enum ParagraphContents {
    Verse(),
    Line(String),
    Character {
        ty: CharacterType,
        contents: Vec<CharacterContents>,
    },
    Footnote {
        style: FootnoteStyle,
        elements: Vec<FootnoteElement>,
    },
    CrossRef {
        elements: Vec<CrossRefElement>,
    },
}

enum ElementContents {
    Line(String),
    Character {
        ty: CharacterType,
        contents: Vec<CharacterContents>,
    },
    Footnote {
        style: FootnoteStyle,
        elements: Vec<FootnoteElement>,
    },
    CrossRef {
        elements: Vec<CrossRefElement>,
    },
}

enum CharacterContents {
    Line(String),
    Character {
        ty: CharacterType,
        contents: Vec<CharacterContents>,
    },
}

struct FootnoteElement {
    style: FootnoteElementStyle,
    contents: CharacterContents,
}

struct CrossRefElement {
    style: CrossRefElementStyle,
    contents: CharacterContents,
}

enum FootnoteElementStyle {
    TranslationQuote,
    AltTranslationQuote,
    Keyword,
    Label,
    Witness,
    Paragraph,
    Text,
    DeuteroText,
    Reference,
}

enum CrossRefElementStyle {
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

enum CharacterType {
    IntroOutline,
    IntroQuote,
    InlineQuote,

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

enum FootnoteStyle {
    Footnote,
    Endnote,
}

enum ParagraphStyle {
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
    Blank,
}

enum PoetryStyle {
    Normal(u8),
    Right,
    Centered,
    AcrosticHeading,
    Embedded(u8),
    Descriptive,
    Blank,
}

enum ElementType {
    MajorTitle(u8),
    MajorTitleEnding(u8),
    MajorSection(u8),
    MajorReference,
    Section(u8),
    Reference,
    Parallel,
    Inline,
    Descriptive,
    Speaker,
    Division(u8),
    PageBreak,
}
