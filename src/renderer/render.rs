use crate::{Level, Stage};

pub trait Renderer {
    fn render(&self, stage: Stage, level: Level, diagnostic: crate::Diagnostic);
}
