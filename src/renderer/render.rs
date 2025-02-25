use crate::{Diagnostic, Level, Stage, files::Files};

/// Renderer for diagnostic reporting.
pub trait Renderer {
    type Error;

    fn render(
        &self,
        stage: Stage,
        level: Level,
        diagnostic: &Diagnostic,
        files: &Files,
    ) -> Result<(), Self::Error>;
}
