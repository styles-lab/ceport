use std::{
    collections::{HashSet, VecDeque},
    sync::{Mutex, OnceLock},
};

use crate::{Diagnostic, Level, Stage};

/// A `fifo` diagnostic reporting caching queue.
pub trait Caching: Send + Sync {
    /// Determines if a diagnostic with specified `stage` and `level` would be cached.
    fn enabled(&self, stage: Stage, level: Level) -> bool;

    /// push one diagnostic into the fifo cachine queue.
    fn cache(&self, stage: Stage, level: Level, diagnostic: Diagnostic);

    /// pop up the diagnostic reporting at the top of the fifo.
    fn pop(&self) -> Option<(Stage, Level, Diagnostic)>;
}

/// An in-memory fifo cachine queue.
#[derive(Default)]
pub struct InMemoryCached {
    /// enabled level.
    level: Level,
    /// enabled stages.
    stages: HashSet<Stage>,
    /// The maximum length of fifo queue.
    max_length: usize,
    /// fifo queue.
    fifo: Mutex<VecDeque<(Stage, Level, Diagnostic)>>,
}

impl InMemoryCached {
    /// Create a new instance by the limits of fifo length.
    pub fn new(max_length: usize) -> Self {
        InMemoryCached {
            max_length,
            ..Default::default()
        }
    }

    /// Reset the enabled level.
    pub fn enable_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Enable a new stage.
    pub fn enable_stage(mut self, stage: Stage) -> Self {
        self.stages.insert(stage);
        self
    }
}

impl Caching for InMemoryCached {
    fn enabled(&self, stage: Stage, level: Level) -> bool {
        if !(u8::from(self.level) < u8::from(level)) {
            return self.stages.contains(&stage);
        }

        return false;
    }

    fn cache(&self, stage: Stage, level: Level, diagnostic: Diagnostic) {
        if self.enabled(stage, level) {
            let mut fifo = self.fifo.lock().unwrap();

            if fifo.len() < self.max_length {
                fifo.push_back((stage, level, diagnostic));
            } else {
                log::warn!("ceport: in-memory caching is full({})", fifo.len());
            }
        }
    }

    fn pop(&self) -> Option<(Stage, Level, Diagnostic)> {
        self.fifo.lock().unwrap().pop_front()
    }
}

static RENDERER: OnceLock<Box<dyn Caching>> = OnceLock::new();

/// Set the diagnostic reporting [`Caching`].
pub fn set_caching<R>(renderer: R)
where
    R: Caching + 'static,
{
    RENDERER
        .set(Box::new(renderer))
        .map_err(|_| ())
        .expect("call set_renderer twice.");
}

/// Get registered diagnostic reporting [`Caching`]
pub fn get_caching() -> &'static dyn Caching {
    RENDERER
        .get()
        .map(|v| v.as_ref())
        .expect("call set_caching firstly to set the global caching instance.")
}

/// logs a diagnostic reporting.
pub fn diagnostic<S, L, B>(stage: S, level: L, builder: B)
where
    Stage: From<S>,
    Level: From<L>,
    B: FnOnce() -> Diagnostic,
{
    let renderer = get_caching();

    let stage = stage.into();
    let level = level.into();
    if renderer.enabled(stage, level) {
        renderer.cache(stage, level, builder());
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
    use crate::Label;

    use super::*;

    static STAGE: Stage = Stage::Parsing("SVG");

    #[test]
    fn test_diagnostic() {
        set_caching(InMemoryCached::new(100));
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
