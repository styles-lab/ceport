use std::{ops::Range, usize};

use crate::sources::SrcId;

/// Optional code for diagnostic reporting.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Code(pub usize);

impl From<i32> for Code {
    fn from(value: i32) -> Self {
        Self(value as usize)
    }
}

/// Reporting level.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Level {
    Bug = 1,
    Error,
    Warn,
}

impl From<Level> for u8 {
    fn from(value: Level) -> Self {
        value as u8
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::Error
    }
}

/// The compilation stage.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Stage {
    None,
    Parsing(&'static str),
    Semantic(&'static str),
    CodeGen(&'static str),
    Custom {
        /// custom stage name,
        stage: &'static str,
        /// diagnostic target name.
        target: &'static str,
    },
}

impl Default for Stage {
    fn default() -> Self {
        Self::None
    }
}

/// Label display style.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LabelStyle {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

/// Addition diagnostic information.
#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    pub file: SrcId,
    pub range: Range<usize>,
    pub style: LabelStyle,
    pub content: String,
}

impl Label {
    /// Create a label with `primary` style.
    pub fn primary<F, R, C>(file: F, range: R, content: C) -> Self
    where
        SrcId: From<F>,
        Range<usize>: From<R>,
        String: From<C>,
    {
        Self {
            file: file.into(),
            range: range.into(),
            style: LabelStyle::Primary,
            content: content.into(),
        }
    }

    /// Create a label with `secondary` style.
    pub fn secondary<F, R, C>(file: F, range: R, content: C) -> Self
    where
        SrcId: From<F>,
        Range<usize>: From<R>,
        String: From<C>,
    {
        Self {
            file: file.into(),
            range: range.into(),
            style: LabelStyle::Secondary,
            content: content.into(),
        }
    }

    /// Create a label with `tertiary` style.
    pub fn tertiary<F, R, C>(file: F, range: R, content: C) -> Self
    where
        SrcId: From<F>,
        Range<usize>: From<R>,
        String: From<C>,
    {
        Self {
            file: file.into(),
            range: range.into(),
            style: LabelStyle::Tertiary,
            content: content.into(),
        }
    }

    /// Create a label with `quaternary` style.
    pub fn quaternary<F, R, C>(file: F, range: R, content: C) -> Self
    where
        SrcId: From<F>,
        Range<usize>: From<R>,
        String: From<C>,
    {
        Self {
            file: file.into(),
            range: range.into(),
            style: LabelStyle::Quaternary,
            content: content.into(),
        }
    }
}

/// A `Diagnostic` reporting for source file.
#[derive(Default)]
pub struct Diagnostic {
    pub code: Option<Code>,
    pub message: String,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}

impl Diagnostic {
    pub fn new<S>(message: S) -> Self
    where
        String: From<S>,
    {
        Self {
            message: message.into(),
            ..Default::default()
        }
    }

    /// With optional reporting [`code`](Code).
    pub fn with_code<C>(mut self, code: C) -> Self
    where
        Code: From<C>,
    {
        self.code = Some(code.into());
        self
    }

    /// Append a new label to this diagnostic.
    pub fn with_label<L>(mut self, label: L) -> Self
    where
        Label: From<L>,
    {
        self.labels.push(label.into());
        self
    }

    /// Append a new note to this diagnostic.
    pub fn with_note<S>(mut self, note: S) -> Self
    where
        String: From<S>,
    {
        self.notes.push(note.into());
        self
    }
}
