//! Render diagnostic reporting to terminal.
//!
//! This is the default renderer of **ceport**.

use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{Diagnostic, GlobalRenderer, Level, Renderer, Stage};

#[allow(unused)]
struct ColorRenderer {
    stage: Stage,
    level: Level,
    diagnostic: crate::Diagnostic,
    stdout: StandardStream,
}

impl ColorRenderer {
    fn new(stage: Stage, level: Level, diagnostic: Diagnostic) -> Self {
        ColorRenderer {
            stage,
            level,
            diagnostic,
            stdout: StandardStream::stdout(ColorChoice::Auto),
        }
    }
    fn render(&mut self) {
        self.render_level();
        self.render_message();
    }

    fn render_message(&mut self) {
        self.stdout
            .set_color(&ColorSpec::new().set_fg(None).set_bold(true))
            .unwrap();
        writeln!(&mut self.stdout, "{}", self.diagnostic.message).unwrap();
    }

    fn render_level(&mut self) {
        match self.level {
            Level::Bug => {
                self.stdout
                    .set_color(&ColorSpec::new().set_fg(Some(Color::Magenta)).set_bold(true))
                    .unwrap();

                write!(&mut self.stdout, "bug").unwrap();
            }
            Level::Error => {
                self.stdout
                    .set_color(&ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))
                    .unwrap();

                write!(&mut self.stdout, "error").unwrap();
            }
            Level::Warn => {
                self.stdout
                    .set_color(&ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
                    .unwrap();

                write!(&mut self.stdout, "warning").unwrap();
            }
        }

        if let Some(code) = self.diagnostic.code {
            write!(&mut self.stdout, "[{:0>6}]", code.0).unwrap();
        }

        write!(&mut self.stdout, ": ").unwrap();
    }
}

pub struct TerminalRenderer;

#[allow(unused)]
impl Renderer for TerminalRenderer {
    fn render(&self, stage: Stage, level: Level, diagnostic: crate::Diagnostic) {
        let mut color_renderer = ColorRenderer::new(stage, level, diagnostic);

        color_renderer.render();
    }
}

#[allow(unused)]
#[cfg(feature = "global")]
impl GlobalRenderer for TerminalRenderer {
    fn enabled(&self, stage: Stage, level: Level) -> bool {
        true
    }
}
