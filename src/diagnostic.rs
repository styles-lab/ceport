use std::{ops::Range, sync::OnceLock};

use crate::renderer::term::TerminalRenderer;

/// A reference to source file.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct FileId(pub usize);

/// Reporting level.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Level {
    Bug = 1,
    Error,
    Warn,
}

impl Default for Level {
    fn default() -> Self {
        Self::Error
    }
}

/// The compilation stage.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Stage {
    Parsing,
    Semantic,
    CodeGen,
    Custom(&'static str),
}

impl Default for Stage {
    fn default() -> Self {
        Self::Parsing
    }
}

impl From<&'static str> for Stage {
    fn from(value: &'static str) -> Self {
        Stage::Custom(value)
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
    pub file: FileId,
    pub range: Range<usize>,
    pub style: LabelStyle,
    pub content: String,
}

impl Label {
    /// Create a label with `primary` style.
    pub fn primary<F, R, C>(file: F, range: R, content: C) -> Self
    where
        FileId: From<F>,
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
        FileId: From<F>,
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
        FileId: From<F>,
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
        FileId: From<F>,
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
    pub stage: Stage,
    pub level: Level,
    pub code: usize,
    pub message: String,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}

impl Diagnostic {
    /// Create a diagnostic with `bug` level.
    pub fn bug<S, C, M>(stage: S, code: C, message: M) -> Self
    where
        Stage: From<S>,
        usize: From<C>,
        String: From<M>,
    {
        Diagnostic {
            stage: stage.into(),
            level: Level::Bug,
            code: code.into(),
            message: message.into(),
            ..Default::default()
        }
    }
    /// Create a diagnostic with `error` level.
    pub fn error<S, C, M>(stage: S, code: C, message: M) -> Self
    where
        Stage: From<S>,
        usize: From<C>,
        String: From<M>,
    {
        Diagnostic {
            stage: stage.into(),
            level: Level::Error,
            code: code.into(),
            message: message.into(),
            ..Default::default()
        }
    }

    /// Create a diagnostic with `warn` level.
    pub fn warn<S, C, M>(stage: S, code: C, message: M) -> Self
    where
        Stage: From<S>,
        usize: From<C>,
        String: From<M>,
    {
        Diagnostic {
            stage: stage.into(),
            level: Level::Warn,
            code: code.into(),
            message: message.into(),
            ..Default::default()
        }
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

/// Diagnostic reporting renderer.
pub trait Renderer: Sync + Send {
    /// Determines if a diagnostic with the specified `level` would be logged.
    fn level_enabled(&self, level: Level) -> bool;

    /// Determines if a diagnostic with the specified `stage` would be logged.
    fn stage_enabled(&self, level: Level) -> bool;

    /// logs the `Diagnostic`
    fn log(&self, diagnostic: Diagnostic);
}

static TERM_RENDERER: &'static dyn Renderer = &TerminalRenderer;
static RENDERER: OnceLock<Box<dyn Renderer>> = OnceLock::new();

/// Set the global [`Renderer`].
pub fn set_renderer<R>(renderer: R)
where
    R: Renderer + 'static,
{
    RENDERER
        .set(Box::new(renderer))
        .map_err(|_| ())
        .expect("call set_renderer twice.");
}

/// Get the global [`Renderer`].
///
/// Default returns a global [`TerminalRenderer`] instance,
/// you can replace it with your own [`Renderer`] by calling the [`set_renderer`] function.
pub fn get_renderer() -> &'static dyn Renderer {
    RENDERER.get().map(|v| v.as_ref()).unwrap_or(TERM_RENDERER)
}
