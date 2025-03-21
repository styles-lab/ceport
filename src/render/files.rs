use std::{fmt::Display, ops::Range};

use crate::FileId;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    /// The line number in the source file.
    pub lines: usize,
    /// The col number in the source file.
    pub cols: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.lines, self.cols)
    }
}

/// The source manager must implement this trait.
pub trait Files {
    /// Convert range into `Location` range.
    fn to_location(&self, id: FileId, range: &Range<usize>) -> Range<Location>;

    /// Read file content by line number.
    fn as_str(&self, id: FileId, lines: usize) -> &str;

    /// Convert file id to file name.
    fn to_file_name(&self, id: FileId) -> &str;
}

/// A source file with line break index.
struct ParsedFile {
    line_break_offsets: Vec<usize>,
    file_name: String,
    content: String,
}

impl ParsedFile {
    fn new(file_name: &str, content: &str) -> Self {
        let mut line_break_offsets = vec![];
        for (idx, c) in content.as_bytes().iter().enumerate() {
            if *c == b'\n' {
                line_break_offsets.push(idx);
            }
        }

        Self {
            line_break_offsets,
            content: content.to_string(),
            file_name: file_name.to_string(),
        }
    }

    fn location(&self, range: &Range<usize>) -> Range<Location> {
        let start = self
            .do_location(range.start)
            .expect(&format!("location(start): out of range {}", range.start));

        let end = self
            .do_location(range.end)
            .expect(&format!("location(end): out of range {}", range.end));

        start..end
    }

    fn as_str(&self, lines: usize) -> &str {
        assert!(lines > 0, "lines must greater than 0.");
        let lines = lines - 1;

        assert!(
            lines < self.line_break_offsets.len() + 1,
            "lines out of range."
        );

        if self.line_break_offsets.is_empty() {
            return self.content.as_str();
        }

        if lines == 0 {
            return &self.content[..self.line_break_offsets[0]];
        }

        return &self.content
            [self.line_break_offsets[lines - 1] + 1..self.line_break_offsets[lines]];
    }

    fn do_location(&self, offset: usize) -> Option<Location> {
        if self.line_break_offsets.is_empty() {
            return Some(Location {
                lines: 1,
                cols: offset + 1,
            });
        }

        for (idx, o) in self.line_break_offsets.iter().enumerate() {
            if offset <= *o {
                if idx != 0 {
                    let cols = offset - self.line_break_offsets[idx - 1] - 1;

                    return Some(Location {
                        lines: idx + 1,
                        cols: cols + 1,
                    });
                }
            }
        }

        None
    }
}

/// A simple in-memory source codes manager.
#[derive(Default)]
pub struct SourceCodes(Vec<ParsedFile>);

impl SourceCodes {
    /// Add a new soure file content.
    pub fn add<N: AsRef<str>, C: AsRef<str>>(&mut self, name: N, content: C) -> FileId {
        let id = self.0.len();

        self.0
            .push(ParsedFile::new(name.as_ref(), content.as_ref()));

        FileId(id)
    }
}

impl Files for SourceCodes {
    fn to_location(&self, id: FileId, range: &Range<usize>) -> Range<Location> {
        assert!(
            id.0 < self.0.len(),
            "InMemoryFiles::location: file id({}) out of range",
            id.0
        );

        let file = &self.0[id.0];

        file.location(range)
    }

    fn as_str(&self, id: FileId, lines: usize) -> &str {
        assert!(
            id.0 < self.0.len(),
            "InMemoryFiles::as_str: file id({}) out of range",
            id.0
        );

        let file = &self.0[id.0];

        file.as_str(lines)
    }

    fn to_file_name(&self, id: FileId) -> &str {
        assert!(
            id.0 < self.0.len(),
            "InMemoryFiles::to_file_name: file id({}) out of range",
            id.0
        );

        let file = &self.0[id.0];

        &file.file_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let file = ParsedFile::new(
            "test",
            unindent::unindent(
                r#"
            module FizzBuzz where

            fizz₁ : Nat → String
            fizz₁ num = case (mod num 5) (mod num 3) of
                0 0 => "FizzBuzz"
                0 _ => "Fizz"
                _ 0 => "Buzz"
                _ _ => num

            fizz₂ : Nat → String
            fizz₂ num =
                case (mod num 5) (mod num 3) of
                    0 0 => "FizzBuzz"
                    0 _ => "Fizz"
                    _ 0 => "Buzz"
                    _ _ => num
        "#,
            )
            .as_str(),
        );

        assert_eq!(file.line_break_offsets.len(), 16);

        assert_eq!(file.as_str(1), "module FizzBuzz where");
        assert_eq!(file.as_str(16), "        _ _ => num");
    }
}
