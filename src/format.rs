use crate::usfm::*;

pub fn format(book: &Book) -> String {
    let capacity = book.contents.len() * 100;
    let mut output = String::with_capacity(capacity);

    for content in &book.contents {
        output.push_str(&format_book_contents(content));
    }

    output
}

fn format_book_contents(content: &BookContents) -> String {
    use BookContents::*;
    match content {
        Id { code, text } => {
            let mut result = format!("\\id {}", code.to_identifier());
            if let Some(text) = text {
                result.push(' ');
                result.push_str(text);
            }
            result.push('\n');
            result
        }
        Usfm(version) => format!("\\usfm {}\n", version),
        Encoding(enc) => format!("\\ide {}\n", encoding_to_str(enc)),
        Status(code) => format!("\\sts {}\n", code),
        Chapter(num) => format!("\\c  {}\n", num),
        AltChapter(num) => format!("\\ca {}\\ca*", num),
        Paragraph(p) => format_paragraph(p),
        Poetry(p) => format_poetry(p),
        Element(e) => format_element(e),
        Empty(ty) => format!("\\{}\n", empty_type_to_str(ty)),
        TableRow(tr) => format_table_row(tr),
        Sidebar(sb) => format_sidebar(sb),
        Peripheral(text) => format!("\\periph {}\n", text),
    }
}

fn encoding_to_str(enc: &BookEncoding) -> &'static str {
    use BookEncoding::*;
    match enc {
        CP1252 => "cp1252",
        CP1251 => "cp1251",
        UTF8 => "UTF-8",
        UTF16 => "UTF-16",
    }
}

fn empty_type_to_str(ty: &EmptyType) -> &'static str {
    use EmptyType::*;
    match ty {
        Blank => "b",
        PageBreak => "pb",
        IntroBlank => "ib",
    }
}

fn paragraph_style_to_str(style: &ParagraphStyle) -> String {
    use ParagraphStyle::*;
    match style {
        Normal => "p".into(),
        Margin => "m".into(),
        Opening => "po".into(),
        Right => "pr".into(),
        Closure => "cls".into(),
        EmbeddedOpening => "pmo".into(),
        Embedded => "pm".into(),
        EmbeddedClosing => "pmc".into(),
        EmbeddedRefrain => "pmr".into(),
        Indented(n) => format!("pi{}", n),
        MarginIndented => "mi".into(),
        MarginIndentedNum(n) => format!("mi{}", n),
        Basic => "nb".into(),
        Centered => "pc".into(),
        HangingIndented(n) => format!("ph{}", n),
        LiturgicalNote => "lit".into(),
        ListHeader => "lh".into(),
        ListFooter => "lf".into(),
        Descriptive => "d".into(),
        ListEntry(n) => format!("li{}", n),
        EmbeddedListEntry(n) => format!("lim{}", n),
    }
}

fn poetry_style_to_str(style: &PoetryStyle) -> String {
    use PoetryStyle::*;
    match style {
        Normal(n) => format!("q{}", n),
        Right => "qr".into(),
        Centered => "qc".into(),
        AcrosticHeading => "qa".into(),
        Embedded(n) => format!("qm{}", n),
        Descriptive => "qd".into(),
    }
}

fn element_type_to_str(ty: &ElementType) -> String {
    use ElementType::*;
    match ty {
        Remark => "rem".into(),
        Header => "h".into(),
        Contents(n) => format!("toc{}", n),
        AltContents(n) => format!("toca{}", n),
        MajorIntro(n) => format!("imt{}", n),
        SectionIntro(n) => format!("is{}", n),
        Intro => "ip".into(),
        IndentedIntro => "ipi".into(),
        MarginIntro => "im".into(),
        MarginIndentedIntro => "imi".into(),
        QuotedIntro => "ipq".into(),
        MarginQuotedIntro => "imq".into(),
        RightIntro => "ipr".into(),
        CenteredIntro => "ipc".into(),
        LiturgicalIntro => "ilit".into(),
        PoetryIntro(n) => format!("iq{}", n),
        ListIntro(n) => format!("ili{}", n),
        OutlineIntro => "iot".into(),
        EntryIntro(n) => format!("io{}", n),
        BridgeIntro => "iex".into(),
        MajorTitleEndingIntro(n) => format!("imte{}", n),
        EndIntro => "ie".into(),
        ChapterLabel => "cl".into(),
        ChapterPublishedLabel => "cp".into(),
        ChapterDescription => "cd".into(),
        MajorTitle(n) => format!("mt{}", n),
        MajorTitleEnding(n) => format!("mte{}", n),
        MajorSection(n) => format!("ms{}", n),
        MajorReference => "mr".into(),
        Section(n) => format!("s{}", n),
        Reference => "sr".into(),
        Parallel => "r".into(),
        Speaker => "sp".into(),
        Division(n) => format!("sd{}", n),
        Restore => "restore".into(),
    }
}

fn character_type_to_str(ty: &CharacterType) -> &'static str {
    use CharacterType::*;
    match ty {
        IntroOutline => "ior",
        IntroQuote => "iqt",
        InlineQuote => "rq",
        PublishedVerse => "vp",
        Selah => "qs",
        AcrosticLetter => "qac",
        Addition => "add",
        BookQuote => "bk",
        DeuteroAddition => "dc",
        Keyword => "k",
        Deity => "nd",
        Ordinal => "ord",
        Proper => "pn",
        Geographic => "png",
        ProperAddition => "addpn",
        QuotedText => "qt",
        Signature => "sig",
        SecondaryText => "sls",
        Transliterated => "tl",
        Jesus => "wj",
        Emphasis => "em",
        Bold => "bd",
        Italic => "it",
        BoldItalic => "bdit",
        Normal => "no",
        SmallCap => "sc",
        Superscipt => "sup",
        Index => "ndx",
        Ruby => "rb",
        Pronunciation => "pro",
        Word => "w",
        GreekWord => "wg",
        HebrewWord => "wh",
        AramaicWord => "wa",
        ForeignWord => "wl",
        Link => "jmp",
        ExtFootnoteRef => "efm",
        FootnoteVerse => "fv",
        FootnoteRef => "fm",
        ListTotal => "litl",
        ListKey => "lik",
        ListValue(_) => "liv",
        InlineSubheading => "pl",
        ScriptureRef => "ref",
        TextAlternative => "ta",
        AltVerse => "va",
        CrossRefTarget => "xt",
    }
}

fn footnote_style_to_str(style: &FootnoteStyle) -> &'static str {
    use FootnoteStyle::*;
    match style {
        Footnote => "f",
        Endnote => "fe",
        ExtendedFootnote => "ef",
        ExtendedEndnote => "efe",
    }
}

fn cross_ref_style_to_str(style: &CrossRefStyle) -> &'static str {
    use CrossRefStyle::*;
    match style {
        CrossRef => "x",
        ExtendedCrossRef => "ex",
    }
}

fn footnote_element_style_to_str(style: &FootnoteElementStyle) -> &'static str {
    use FootnoteElementStyle::*;
    match style {
        Reference => "fr",
        TranslationQuote => "fq",
        AltTranslationQuote => "fqa",
        Keyword => "fk",
        Label => "fl",
        Witness => "fw",
        Paragraph => "fp",
        Text => "ft",
        DeuteroText => "fdc",
        Verse => "fv",
    }
}

fn cross_ref_element_style_to_str(style: &CrossRefElementStyle) -> &'static str {
    use CrossRefElementStyle::*;
    match style {
        Keyword => "xk",
        Quote => "xq",
        Target => "xt",
        ExtraTarget => "xta",
        Origin => "xop",
        OldTarget => "xot",
        NewTarget => "xnt",
        DeuteroTarget => "xdc",
        InlineQuote => "rq",
        OriginRef => "xo",
    }
}

fn cell_prefix_to_str(prefix: &CellPrefix) -> &'static str {
    use CellPrefix::*;
    match prefix {
        Header => "th",
        HeaderRight => "thr",
        HeaderCenter => "thc",
        Content => "tc",
        ContentRight => "tcr",
        ContentCenter => "tcc",
    }
}

fn format_caller(caller: &Caller) -> String {
    use Caller::*;
    match caller {
        Auto => " +".into(),
        None => " -".into(),
        Some(c) => format!(" {}", c),
    }
}

fn format_paragraph(p: &Paragraph) -> String {
    let style = paragraph_style_to_str(&p.style);
    let mut result = format!("\\{}", style);

    let contents = format_paragraph_contents(&p.contents);
    if !contents.is_empty() {
        result.push(' ');
        result.push_str(&contents);
    }

    result.push('\n');
    result
}

fn format_poetry(p: &Poetry) -> String {
    let style = poetry_style_to_str(&p.style);
    let mut result = format!("\\{}", style);

    let contents = format_paragraph_contents(&p.contents);
    if !contents.is_empty() {
        result.push(' ');
        result.push_str(&contents);
    }

    result.push('\n');
    result
}

fn format_element(e: &Element) -> String {
    let ty = element_type_to_str(&e.ty);
    let mut result = format!("\\{}", ty);

    let contents = format_element_contents(&e.contents);
    if !contents.is_empty() {
        result.push(' ');
        result.push_str(&contents);
    }

    result.push('\n');
    result
}

fn format_paragraph_contents(contents: &[ParagraphContents]) -> String {
    let mut result = String::new();
    for content in contents {
        use ParagraphContents::*;
        match content {
            Verse(verse) => {
                result.push('\n');
                result.push_str(&format_verse(verse));
            }
            Line(text) => {
                result.push_str(text);
            }
            Character(c) => {
                result.push_str(&format_character(c, false));
            }
            Footnote(f) => {
                result.push_str(&format_footnote(f));
            }
            CrossRef(x) => {
                result.push_str(&format_cross_ref(x));
            }
            Figure(fig) => {
                result.push_str(&format_figure(fig));
            }
            Milestone(ms) => {
                result.push_str(&format_milestone(ms));
            }
            Category(cat) => {
                result.push_str(&format!("\\cat {}\\cat*", cat));
            }
            OptionalBreak => {
                result.push_str("//");
            }
        }
    }
    result
}

fn format_element_contents(contents: &[ElementContents]) -> String {
    let mut result = String::new();
    for content in contents {
        use ElementContents::*;
        match content {
            Line(text) => {
                result.push_str(text);
            }
            Character(c) => {
                result.push_str(&format_character(c, false));
            }
            Footnote(f) => {
                result.push_str(&format_footnote(f));
            }
            CrossRef(x) => {
                result.push_str(&format_cross_ref(x));
            }
            Figure(fig) => {
                result.push_str(&format_figure(fig));
            }
            Milestone(ms) => {
                result.push_str(&format_milestone(ms));
            }
            Category(cat) => {
                result.push_str(&format!("\\cat {}\\cat*", cat));
            }
            OptionalBreak => {
                result.push_str("//");
            }
        }
    }
    result
}

fn format_character_contents(contents: &[CharacterContents]) -> String {
    format_character_contents_impl(contents, true)
}

fn format_character_contents_impl(contents: &[CharacterContents], nest: bool) -> String {
    let mut result = String::new();
    for content in contents {
        use CharacterContents::*;
        match content {
            Line(text) => {
                result.push_str(text);
            }
            Character(c) => {
                result.push_str(&format_character(c, nest));
            }
            Footnote(f) => {
                result.push_str(&format_footnote(f));
            }
            CrossRef(x) => {
                result.push_str(&format_cross_ref(x));
            }
            Figure(fig) => {
                result.push_str(&format_figure(fig));
            }
            Milestone(ms) => {
                result.push_str(&format_milestone(ms));
            }
            OptionalBreak => {
                result.push_str("//");
            }
        }
    }
    result
}

fn format_character(c: &Character, is_nested: bool) -> String {
    let prefix = if is_nested { "\\+" } else { "\\" };
    let base_ty = character_type_to_str(&c.ty);
    let ty = match &c.ty {
        CharacterType::ListValue(n) if *n > 0 => format!("{}{}", base_ty, n),
        _ => base_ty.to_string(),
    };

    let mut result = format!(
        "{}{}{}",
        prefix,
        ty,
        if c.contents.is_empty() && c.attributes.is_empty() {
            ""
        } else {
            " "
        }
    );

    let contents = format_character_contents(&c.contents);
    if !contents.is_empty() {
        result.push_str(&contents);
    }

    if !c.attributes.is_empty() {
        result.push_str(&format_attributes(&c.attributes));
    }

    result.push_str(&format!("{}{}*", prefix, ty));
    result
}

fn format_attributes(attrs: &[(String, String)]) -> String {
    if attrs.is_empty() {
        return String::new();
    }

    // Check if it's a single default value attribute (parsed as "lemma" key)
    if attrs.len() == 1 && attrs[0].0 == "lemma" {
        return format!("|{}", attrs[0].1);
    }

    let mut result = String::from("|");
    for (i, (key, value)) in attrs.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(&format!("{}=\"{}\"", key, value));
    }
    result
}

fn format_verse(verse: &str) -> String {
    format!("\\v {} ", verse)
}

fn format_footnote(f: &Footnote) -> String {
    let style = footnote_style_to_str(&f.style);
    let caller = format_caller(&f.caller);

    let mut result = format!(
        "\\{}{}{}",
        style,
        caller,
        if f.elements.is_empty() { "" } else { " " }
    );

    for elem in &f.elements {
        let elem_style = footnote_element_style_to_str(&elem.style);
        let contents = format_character_contents_impl(&elem.contents, false);
        result.push_str(&format!("\\{} {}", elem_style, contents));
    }

    result.push_str(&format!("\\{}*", style));
    result
}

fn format_cross_ref(x: &CrossRef) -> String {
    let style = cross_ref_style_to_str(&x.style);
    let caller = format_caller(&x.caller);

    let mut result = format!(
        "\\{}{}{}",
        style,
        caller,
        if x.elements.is_empty() { "" } else { " " }
    );

    for elem in &x.elements {
        let elem_style = cross_ref_element_style_to_str(&elem.style);
        let contents = format_character_contents_impl(&elem.contents, false);
        result.push_str(&format!("\\{} {}", elem_style, contents));
    }

    result.push_str(&format!("\\{}*", style));
    result
}

fn format_figure(f: &Figure) -> String {
    let mut result = String::from("\\fig ");

    let contents = format_character_contents_impl(&f.contents, false);
    if !contents.is_empty() {
        result.push_str(&contents);
    }

    if !f.attributes.is_empty() {
        result.push_str(&format_attributes(&f.attributes));
    }

    result.push_str("\\fig*");
    result
}

fn format_milestone(ms: &Milestone) -> String {
    use MilestoneStyle::*;

    let (style, bound) = match &ms.style {
        QuotedText(n, b) => (format!("qt{}", n), Some(b)),
        TextSection(b) => ("ts".to_string(), Some(b)),
        Text(b) => ("t".to_string(), Some(b)),
        WordsOfJesus(b) => ("wj".to_string(), Some(b)),
        VerseId => ("vid".to_string(), None),
    };

    let mut result = format!("\\{}", style);

    if let Some(bound) = bound {
        match bound {
            MilestoneBound::Start => result.push_str("-s"),
            MilestoneBound::End => result.push_str("-e"),
            MilestoneBound::None => {}
        }
    }

    if !ms.attributes.is_empty() {
        result.push_str(&format_attributes(&ms.attributes));
    }

    result.push_str("\\*");
    result
}

fn format_table_row(tr: &TableRow) -> String {
    let mut result = String::from("\\tr ");

    for cell in &tr.cells {
        let prefix = cell_prefix_to_str(&cell.prefix);
        let contents = format_paragraph_contents(&cell.contents);
        result.push_str(&format!("\n\\{}{} {}", prefix, cell.column, contents));
    }

    result.push('\n');
    result
}

fn format_sidebar(s: &Sidebar) -> String {
    let mut result = String::from("\\esb\n");

    for content in &s.contents {
        use SidebarContents::*;
        match content {
            Paragraph(p) => result.push_str(&format_paragraph(p)),
            Poetry(p) => result.push_str(&format_poetry(p)),
            Element(e) => result.push_str(&format_element(e)),
            Empty(ty) => result.push_str(&format!("\\{}\n", empty_type_to_str(ty))),
            TableRow(tr) => result.push_str(&format_table_row(tr)),
            Category(cat) => result.push_str(&format!("\\cat {}\\cat*\n", cat)),
        }
    }

    result.push_str("\\esbe\n");
    result
}
