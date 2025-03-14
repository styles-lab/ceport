//! A terminal renderer implementation.

use super::Renderer;

/// A diagnostic reporting renderer implementation that renders the result to the terminal.
pub struct Term;

impl Renderer for Term {
    type Error = std::io::Error;

    fn render<'a, D>(&self, _: D) -> Result<(), Self::Error>
    where
        crate::Diagnostic<'a>: From<D>,
    {
        todo!()
    }
}
