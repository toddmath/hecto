pub struct FileType<'a> {
    name: String,
    hl_opts: HighlightingOptions<'a>,
}

#[derive(Default)]
pub struct HighlightingOptions<'a> {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: &'a [&'a str],
    secondary_keywords: &'a [&'a str],
}

impl<'a> Default for FileType<'a> {
    fn default() -> Self {
        Self {
            name: "No filetype".into(),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl<'a> From<&str> for FileType<'a> {
    fn from(ft: &str) -> Self {
        if ft.ends_with(".rs") {
            Self {
                name: "Rust".into(),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: &[
                        "as", "break", "const", "continue", "crate", "else",
                        "enum", "extern", "false", "fn", "for", "if", "impl",
                        "in", "let", "loop", "match", "mod", "move", "mut",
                        "pub", "ref", "return", "self", "Self", "static",
                        "struct", "super", "trait", "true", "type", "unsafe",
                        "use", "where", "while", "dyn", "abstract", "become",
                        "box", "do", "final", "macro", "override", "priv",
                        "typeof", "unsized", "virtual", "yield", "async",
                        "await", "try",
                    ],
                    secondary_keywords: &[
                        "bool", "char", "i8", "i16", "i32", "i64", "isize",
                        "u8", "u16", "u32", "u64", "usize", "f32", "f64",
                    ],
                },
            }
        } else {
            Self::default()
        }
    }
}

impl<'a> FileType<'a> {
    #[inline]
    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }

    #[inline]
    pub(crate) const fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_opts
    }
}

impl<'a> HighlightingOptions<'a> {
    #[inline]
    pub(crate) const fn numbers(&self) -> bool {
        self.numbers
    }

    #[inline]
    pub(crate) const fn strings(&self) -> bool {
        self.strings
    }

    #[inline]
    pub(crate) const fn characters(&self) -> bool {
        self.characters
    }

    #[inline]
    pub(crate) const fn comments(&self) -> bool {
        self.comments
    }

    #[inline]
    pub(crate) const fn primary_keywords(&self) -> &[&str] {
        self.primary_keywords
    }

    #[inline]
    pub(crate) const fn secondary_keywords(&self) -> &[&str] {
        self.secondary_keywords
    }

    #[inline]
    pub(crate) const fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }
}
