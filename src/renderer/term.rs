//! Render diagnostic reporting to terminal.
//!
//! This is the default renderer of **ceport**.

use crate::Renderer;

pub struct TerminalRenderer;

#[allow(unused)]
impl Renderer for TerminalRenderer {
    fn level_enabled(&self, level: crate::Level) -> bool {
        todo!()
    }

    fn stage_enabled(&self, level: crate::Level) -> bool {
        todo!()
    }

    fn log(&self, diagnostic: crate::Diagnostic) {
        todo!()
    }
}
