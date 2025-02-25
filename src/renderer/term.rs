//! Render diagnostic reporting to terminal.
//!
//! This is the default renderer of **ceport**.

use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{
    Diagnostic, Level, Stage,
    files::{Files, SrcId},
};

use super::Renderer;

#[allow(unused)]
struct ColorRenderer<'a> {
    stage: Stage,
    level: Level,
    diagnostic: crate::Diagnostic,
    stdout: StandardStream,
    files: &'a Files,
}

impl<'a> ColorRenderer<'a> {
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

    fn write_file_name(&mut self, id: SrcId) {
        self.stdout.set_color(&Self::border_color_spec()).unwrap();

        writeln!(
            &mut self.stdout,
            "   ┌─ {}",
            self.files
                .get(id)
                .expect(&format!("unknown file id {:?}", id))
                .name()
        )
        .unwrap();
    }
}

impl<'a> ColorRenderer<'a> {
    fn new(stage: Stage, level: Level, diagnostic: Diagnostic, files: &'a Files) -> Self {
        ColorRenderer {
            stage,
            level,
            diagnostic,
            files,
            stdout: StandardStream::stdout(ColorChoice::Auto),
        }
    }
    fn render(&mut self) {
        self.render_level();
        self.render_message();

        self.render_snippet();

        self.stdout.set_color(&Self::text_color_spec()).unwrap();
    }

    fn render_snippet(&mut self) {
        for label in self.diagnostic.labels.clone() {
            self.write_file_name(label.file);
        }
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
    fn render(
        &self,
        stage: Stage,
        level: Level,
        diagnostic: &Diagnostic,
        files: &Files,
    ) -> Result<(), ()> {
        let mut color_renderer = ColorRenderer::new(stage, level, diagnostic.clone(), files);

        color_renderer.render();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Diagnostic, Label, Level, Stage,
        files::{Files, src},
        renderer::Renderer,
    };

    use super::TerminalRenderer;

    #[test]
    fn test_term() {
        let mut files = Files::default();

        files.add(src(
            "FizzBuzz.fun",
            unindent::unindent(
                r#"
            module FizzBuzz where

            fizz₁ : Nat → String
            fizz₁ num = case (mod num 5) (mod num 3) of
                0 0 => "FizzBuzz"
                0 _ => "Fizz"
                _ 0 => "Buzz"
                _ _ => num

            fizz₂ : Nat → String
            fizz₂ num =
                case (mod num 5) (mod num 3) of
                    0 0 => "FizzBuzz"
                    0 _ => "Fizz"
                    _ 0 => "Buzz"
                    _ _ => num
        "#,
            ),
        ));

        let diagnostic = Diagnostic::new("`~` cannot be used as a unary operator")
            .with_code(10)
            .with_note(
                "
                    expected type `String`
                    found type `Nat`
                ",
            )
            .with_label(Label::primary(
                0,
                328..331,
                "expected `String`, found `Nat`",
            ))
            .with_label(Label::primary(
                0,
                211..331,
                "`case` clauses have incompatible types",
            ))
            .with_label(Label::primary(
                0,
                258..268,
                "this is found to be of type `String`",
            ))
            .with_label(Label::primary(
                0,
                284..290,
                "this is found to be of type `String`",
            ))
            .with_label(Label::primary(
                0,
                306..312,
                "this is found to be of type `String`",
            ))
            .with_label(Label::primary(
                0,
                186..192,
                "expected type `String` found here",
            ));

        TerminalRenderer
            .render(Stage::Parsing("test"), Level::Bug, &diagnostic, &files)
            .unwrap();
        TerminalRenderer
            .render(Stage::Parsing("test"), Level::Error, &diagnostic, &files)
            .unwrap();
        TerminalRenderer
            .render(Stage::Parsing("test"), Level::Warn, &diagnostic, &files)
            .unwrap();
    }
}
