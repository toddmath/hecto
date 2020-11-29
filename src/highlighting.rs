use crate::{highlighting, HighlightingOptions};
use termion::color;

#[derive(PartialEq, Copy, Clone)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeyword,
    SecondaryKeyword,
}

impl Type {
    pub(crate) fn to_color(&self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::Match => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(211, 54, 130),
            Type::Character => color::Rgb(108, 113, 196),
            Type::Comment | Type::MultilineComment => color::Rgb(133, 153, 0),
            Type::PrimaryKeyword => color::Rgb(181, 137, 0),
            Type::SecondaryKeyword => color::Rgb(42, 161, 152),
            _ => color::Rgb(255, 255, 255),
        }
    }

    pub(crate) fn fg_string(&self) -> String {
        format!("{}", termion::color::Fg(self.to_color()))
    }
}

pub trait Highlighter {
    fn highlight(
        &mut self,
        opts: &HighlightingOptions,
        word: Option<&str>,
        start_with_comment: bool,
    ) -> bool;

    fn highlight_match(&mut self, word: Option<&str>);

    fn highlight_char(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool;

    fn highlight_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool;

    fn highlight_string(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool;

    fn highlight_number(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool;

    fn highlight_str(
        &mut self,
        index: &mut usize,
        substring: &str,
        chars: &[char],
        hl_type: highlighting::Type,
    ) -> bool;

    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool;

    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool;

    fn highlight_keywords(
        &mut self,
        index: &mut usize,
        chars: &[char],
        keywords: &[&str],
        hl_type: highlighting::Type,
    ) -> bool;

    fn highlight_multiline_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool;
}
