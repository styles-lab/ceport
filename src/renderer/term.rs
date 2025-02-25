//! Render diagnostic reporting to terminal.
//!
//! This is the default renderer of **ceport**.

use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{Diagnostic, Level, Stage};

use super::Renderer;

#[allow(unused)]
struct ColorRenderer {
    stage: Stage,
    level: Level,
    diagnostic: crate::Diagnostic,
    stdout: StandardStream,
}

impl ColorRenderer {
    fn text_color_spec() -> ColorSpec {
        let mut color_spec = ColorSpec::new();

        color_spec.set_fg(None).set_bold(false);

        color_spec
    }

    fn bug_color_spec() -> ColorSpec {
        let mut color_spec = ColorSpec::new();

        color_spec.set_fg(Some(Color::Magenta)).set_bold(true);

        color_spec
    }

    fn error_color_spec() -> ColorSpec {
        let mut color_spec = ColorSpec::new();

        color_spec.set_fg(Some(Color::Red)).set_bold(true);

        color_spec
    }

    fn warn_color_spec() -> ColorSpec {
        let mut color_spec = ColorSpec::new();

        color_spec.set_fg(Some(Color::Yellow)).set_bold(true);

        color_spec
    }

    fn border_color_spec() -> ColorSpec {
        let mut color_spec = ColorSpec::new();

        color_spec.set_fg(Some(Color::Blue)).set_bold(false);

        color_spec
    }

    fn write_left_gutter(&mut self) {
        self.stdout.set_color(&Self::border_color_spec()).unwrap();

        writeln!(&mut self.stdout, "   ┌─ test:9:0").unwrap();
    }
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

        self.write_left_gutter();
        self.write_left_gutter();
        self.write_left_gutter();
        self.write_left_gutter();

        self.stdout.set_color(&Self::text_color_spec()).unwrap();
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
                self.stdout.set_color(&Self::bug_color_spec()).unwrap();

                write!(&mut self.stdout, "  bug").unwrap();
            }
            Level::Error => {
                self.stdout.set_color(&Self::error_color_spec()).unwrap();

                write!(&mut self.stdout, "error").unwrap();
            }
            Level::Warn => {
                self.stdout.set_color(&Self::warn_color_spec()).unwrap();

                write!(&mut self.stdout, " warn").unwrap();
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
    type Error = ();
    fn render(&self, stage: Stage, level: Level, diagnostic: &Diagnostic) -> Result<(), ()> {
        let mut color_renderer = ColorRenderer::new(stage, level, diagnostic.clone());

        color_renderer.render();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Diagnostic, Label, Level, Stage, renderer::Renderer};

    use super::TerminalRenderer;

    #[test]
    fn test_term() {
        let diagnostic = Diagnostic::new("`~` cannot be used as a unary operator")
            .with_code(10)
            .with_note("")
            .with_label(Label::primary(1, 0..100, "hello world"))
            .with_label(Label::primary(1, 0..100, "hello world"))
            .with_label(Label::primary(1, 0..100, "hello world"));

        TerminalRenderer
            .render(Stage::Parsing("test"), Level::Bug, &diagnostic)
            .unwrap();
        TerminalRenderer
            .render(Stage::Parsing("test"), Level::Error, &diagnostic)
            .unwrap();
        TerminalRenderer
            .render(Stage::Parsing("test"), Level::Warn, &diagnostic)
            .unwrap();
    }
}
