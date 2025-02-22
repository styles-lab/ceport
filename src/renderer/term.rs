//! Render diagnostic reporting to terminal.
//!
//! This is the default renderer of **ceport**.

use crate::{Level, Renderer, Stage};

pub struct TerminalRenderer;

#[allow(unused)]
impl Renderer for TerminalRenderer {
    fn render(&self, stage: Stage, level: Level, diagnostic: crate::Diagnostic) {
        todo!()
    }

    fn enabled(&self, stage: crate::Stage, level: crate::Level) -> bool {
        false
    }
}
