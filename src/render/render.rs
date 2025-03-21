use crate::Diagnostic;

use super::Files;

/// A diagnostic reporting renderer must implement this trait.
pub trait Renderer {
    /// Error type returns by [`render`] function when some error occur.
    type Error;

    /// Render a diagnostic.
    fn render<'a, F, D>(&mut self, files: &F, diagnostic: D) -> Result<(), Self::Error>
    where
        F: Files,
        Diagnostic<'a>: From<D>;
}
