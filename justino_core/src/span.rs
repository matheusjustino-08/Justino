//! Span tracking module for `.jucode` source position tracking.

use std::fmt;

/// Represents a source code region within a `.jucode` file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Span {
    /// Identifier for the source file.
    pub file_id: usize,
    /// Absolute byte start offset in the source string.
    pub start: usize,
    /// Absolute byte end offset in the source string.
    pub end: usize,
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number (character offset in line).
    pub column: usize,
}

impl Span {
    /// Creates a new Span instance.
    pub const fn new(file_id: usize, start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            file_id,
            start,
            end,
            line,
            column,
        }
    }

    /// Creates a dummy/empty Span used for synthesized nodes or default initializers.
    pub const fn dummy() -> Self {
        Self {
            file_id: 0,
            start: 0,
            end: 0,
            line: 1,
            column: 1,
        }
    }

    /// Combines two Spans into a single bounding Span covering both.
    pub fn merge(&self, other: &Self) -> Self {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        let (line, column) = if self.start <= other.start {
            (self.line, self.column)
        } else {
            (other.line, other.column)
        };

        Self {
            file_id: self.file_id,
            start,
            end,
            line,
            column,
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "file #{} [{}:{}] (bytes {}-{})",
            self.file_id, self.line, self.column, self.start, self.end
        )
    }
}
