use crate::Diagnostic;

/// A diagnostic reporting renderer must implement this trait.
pub trait Renderer {
    /// Error type returns by [`render`] function when some error occur.
    type Error;

    /// Render a diagnostic.
    fn render<'a, D>(&self, diagnostic: D) -> Result<(), Self::Error>
    where
        Diagnostic<'a>: From<D>;
}
