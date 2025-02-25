use crate::{Diagnostic, Level, Stage};

/// Renderer for diagnostic reporting.
pub trait Renderer {
    type Error;

    fn render(
        &self,
        stage: Stage,
        level: Level,
        diagnostic: &Diagnostic,
    ) -> Result<(), Self::Error>;
}
