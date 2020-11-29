use crate::{FileType, Highlighter, Position, Row, SearchDirection};
use anyhow::Result;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Default)]
pub struct Document<'a> {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
    file_type: FileType<'a>,
}

impl<'a> Document<'a> {
    pub fn open(filename: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(filename)?;
        let file_type = FileType::from(filename);
        // let mut start_with_comment = false;
        let mut rows = vec![];

        for value in contents.lines() {
            // let mut row = Row::from(value);
            rows.push(Row::from(value));
            // start_with_comment = row.highlight(
            //     &file_type.highlighting_options(),
            //     None,
            //     start_with_comment,
            // );
            // rows.push(row);
        }

        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
            file_type,
        })
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }

        #[allow(clippy::indexing_slicing)]
        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);

        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }

    pub(crate) fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;

        if c == '\n' {
            self.insert_newline(at);
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }

        self.unhighlight_rows(at.y);
    }

    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
    pub(crate) fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y >= len {
            return;
        }

        self.dirty = true;

        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }

        self.unhighlight_rows(at.y);
    }

    pub fn save(&mut self) -> Result<(), io::Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            self.file_type = FileType::from(file_name.as_str());

            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }

            self.dirty = false;
        }
        Ok(())
    }

    pub(crate) const fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[allow(clippy::indexing_slicing)]
    pub(crate) fn find(
        &self,
        query: &str,
        at: &Position,
        direction: SearchDirection,
    ) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }

        let mut position = Position::new(at.x, at.y);
        let (start, end) = match direction {
            SearchDirection::Forward => (at.y, self.rows.len()),
            SearchDirection::Backward => (0, at.y.saturating_add(1)),
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                match direction {
                    SearchDirection::Forward => {
                        position.y = position.y.saturating_add(1);
                        position.x = 0;
                    },
                    SearchDirection::Backward => {
                        position.y = position.y.saturating_sub(1);
                        position.x = self.rows[position.y].len();
                    },
                }
            } else {
                return None;
            }
        }

        None
    }

    pub(crate) fn highlight(
        &mut self,
        word: Option<&str>,
        until: Option<usize>,
    ) {
        let mut start_with_comment = false;

        let until = if let Some(until) = until {
            if until.saturating_add(1) < self.rows.len() {
                until.saturating_add(1)
            } else {
                self.rows.len()
            }
        } else {
            self.rows.len()
        };

        #[allow(clippy::indexing_slicing)]
        for row in &mut self.rows[..until] {
            start_with_comment = row.highlight(
                &self.file_type.highlighting_options(),
                word,
                start_with_comment,
            );
        }
    }

    pub(crate) fn unhighlight_rows(&mut self, start: usize) {
        let start = start.saturating_sub(1);

        for row in self.rows.iter_mut().skip(start) {
            row.is_highlighted = false;
        }
    }
}

// impl From<&str> for Document {
//     fn from(value: &str) -> Self {
//         let mut rows = vec![];

//         for v in value.lines() {
//             rows.push(Row::from(v));
//         }

//         Self { rows }
//     }
// }
