use std::{borrow::Cow, ops::Range};

/// A reference to a source code.
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Copy)]
pub struct FileId(pub usize);

impl From<usize> for FileId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// Severity of diagnostic reporting.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Level {
    /// An unexpected bug.
    Bug,
    /// An error.
    Error,
    /// A warning
    Warning,
    /// A note.
    Note,
    /// A help message.
    Help,
}
/// Region of one label.
#[derive(Debug, Clone)]
pub struct LabelRegion<'a> {
    /// The region of code associated with a diagnostic.
    pub range: Range<usize>,
    /// describing of the region.
    pub message: Cow<'a, str>,
}

/// A label describing an underlined region of code associated with a diagnostic.
#[derive(Debug, Clone)]
pub struct Label<'a> {
    /// The file that associated with diagnostic reporting.
    pub id: FileId,
    /// primary region.
    pub primary: LabelRegion<'a>,
    /// Secondary regions.
    pub secondary: Vec<LabelRegion<'a>>,
}

impl<'a> Label<'a> {
    /// Create a new `Label` with primary describing region.
    pub fn new<ID, R, M>(id: ID, range: R, message: M) -> Self
    where
        FileId: From<ID>,
        Range<usize>: From<R>,
        Cow<'a, str>: From<M>,
    {
        Self {
            id: id.into(),
            primary: LabelRegion {
                range: range.into(),
                message: message.into(),
            },
            secondary: vec![],
        }
    }

    /// Append a new secondary describing region.
    pub fn with_secondary<R, M>(mut self, range: R, message: M) -> Self
    where
        Range<usize>: From<R>,
        Cow<'a, str>: From<M>,
    {
        self.secondary.push(LabelRegion {
            range: range.into(),
            message: message.into(),
        });

        self
    }
}

/// A diagnostic reporting instance.
#[derive(Debug, Clone)]
pub struct Diagnostic<'a> {
    /// Severity of this diagnostic reporting.
    pub level: Level,
    /// An optional code the identifies this diagnostic.
    pub code: Option<usize>,
    /// The main message associated with this diagnostic.
    pub message: Cow<'a, str>,
    /// Notes that are associated with the primary cause of the diagnostic.
    pub nodes: Vec<Cow<'a, str>>,

    pub labels: Vec<Label<'a>>,
}

impl<'a> Diagnostic<'a> {
    /// Create a new diagnostic.
    pub fn new<M>(level: Level, message: M) -> Self
    where
        Cow<'a, str>: From<M>,
    {
        Self {
            level,
            code: None,
            message: message.into(),
            nodes: vec![],
            labels: vec![],
        }
    }
    /// Create a new diagnostic with a severity of [`Bug`](Level::Bug)
    pub fn bug<M>(message: M) -> Self
    where
        Cow<'a, str>: From<M>,
    {
        Self::new(Level::Bug, message)
    }
    /// Create a new diagnostic with a severity of [`Error`](Level::Error)
    pub fn error<M>(message: M) -> Self
    where
        Cow<'a, str>: From<M>,
    {
        Self::new(Level::Error, message)
    }

    /// Create a new diagnostic with a severity of [`Warning`](Level::Warning)
    pub fn warning<M>(message: M) -> Self
    where
        Cow<'a, str>: From<M>,
    {
        Self::new(Level::Warning, message)
    }

    /// Create a new diagnostic with a severity of [`Note`](Level::Note)
    pub fn note<M>(message: M) -> Self
    where
        Cow<'a, str>: From<M>,
    {
        Self::new(Level::Note, message)
    }
    /// Create a new diagnostic with a severity of [`Help`](Level::Help)
    pub fn help<M>(message: M) -> Self
    where
        Cow<'a, str>: From<M>,
    {
        Self::new(Level::Help, message)
    }

    /// Set optional code.
    pub fn with_code(mut self, code: usize) -> Self {
        self.code = Some(code);
        self
    }

    /// Add some notes to the diagnostic.
    pub fn with_note<M>(mut self, message: M) -> Self
    where
        Cow<'a, str>: From<M>,
    {
        self.nodes.push(message.into());
        self
    }

    /// Add some labels to the diagnostic.
    pub fn with_label<L>(mut self, label: L) -> Self
    where
        Label<'a>: From<L>,
    {
        self.labels.push(label.into());
        self
    }
}
