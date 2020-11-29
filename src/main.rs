#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::must_use_candidate,
    clippy::missing_errors_doc
)]
mod document;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use editor::{Position, SearchDirection};
pub use filetype::{FileType, HighlightingOptions};
pub use highlighting::Highlighter;
pub use row::Row;
pub use terminal::Terminal;

use anyhow::Result;

fn main() -> Result<()> {
    Editor::new()?.run();
    Ok(())
}
