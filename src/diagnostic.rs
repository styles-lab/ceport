use std::{ops::Range, sync::OnceLock, usize};

use crate::{renderer::term::TerminalRenderer, sources::SrcId};

/// Optional code for diagnostic reporting.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Code(pub usize);

impl From<i32> for Code {
    fn from(value: i32) -> Self {
        Self(value as usize)
    }
}

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

/// Diagnostic reporting renderer.
pub trait Renderer: Sync + Send {
    /// logs the `Diagnostic`
    fn render(&self, stage: Stage, level: Level, diagnostic: Diagnostic);
}

#[cfg(feature = "global")]
mod global {
    use super::*;
    /// Diagnostic reporting renderer.
    pub trait GlobalRenderer: Renderer {
        /// Determines if a diagnostic with specified `stage` and `level` would be logged.
        fn enabled(&self, stage: Stage, level: Level) -> bool;
    }

    static TERM_RENDERER: &'static dyn GlobalRenderer = &TerminalRenderer;
    static RENDERER: OnceLock<Box<dyn GlobalRenderer>> = OnceLock::new();

    /// Set the global [`Renderer`].
    pub fn set_renderer<R>(renderer: R)
    where
        R: GlobalRenderer + 'static,
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
    pub fn get_renderer() -> &'static dyn GlobalRenderer {
        RENDERER.get().map(|v| v.as_ref()).unwrap_or(TERM_RENDERER)
    }

    /// logs a diagnostic reporting.
    pub fn diagnostic<S, L, B>(stage: S, level: L, builder: B)
    where
        Stage: From<S>,
        Level: From<L>,
        B: FnOnce() -> Diagnostic,
    {
        let renderer = get_renderer();

        let stage = stage.into();
        let level = level.into();
        if renderer.enabled(stage, level) {
            renderer.render(stage, level, builder());
        }
    }

    /// Report a bug.
    pub fn bug<S, B>(stage: S, builder: B)
    where
        Stage: From<S>,
        B: FnOnce() -> Diagnostic,
    {
        diagnostic(stage, Level::Bug, builder);
    }

    /// Report a error.
    pub fn error<S, B>(stage: S, builder: B)
    where
        Stage: From<S>,
        B: FnOnce() -> Diagnostic,
    {
        diagnostic(stage, Level::Error, builder);
    }

    /// Report a warn.
    pub fn warn<S, B>(stage: S, builder: B)
    where
        Stage: From<S>,
        B: FnOnce() -> Diagnostic,
    {
        diagnostic(stage, Level::Warn, builder);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        static STAGE: Stage = Stage::Parsing("SVG");

        #[test]
        fn test_diagnostic() {
            error(STAGE, || {
                Diagnostic::new("hello world")
                    .with_code(10)
                    .with_note("")
                    .with_label(Label::primary(1, 0..100, "hello world"))
                    .with_label(Label::primary(1, 0..100, "hello world"))
                    .with_label(Label::primary(1, 0..100, "hello world"))
            });
        }
    }
}

#[cfg(feature = "global")]
pub use global::*;
