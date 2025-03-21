//! A terminal renderer implementation.

use std::{
    collections::{HashMap, HashSet},
    io::{Result, Write},
};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use unicode_width::UnicodeWidthStr;

use crate::{Diagnostic, Label, Level};

use super::{Files, Renderer};

/// A diagnostic reporting renderer implementation that renders the result to the terminal.
pub struct Term(StandardStream);

impl Default for Term {
    fn default() -> Self {
        Self(StandardStream::stdout(ColorChoice::Always))
    }
}

impl Term {
    fn error_color(&mut self) -> Result<()> {
        self.0
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)))
    }

    fn bug_color(&mut self) -> Result<()> {
        self.0
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Magenta)))
    }

    fn warn_color(&mut self) -> Result<()> {
        self.0
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Yellow)))
    }

    fn text_color(&mut self) -> Result<()> {
        self.0
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::White)))
    }

    fn help_color(&mut self) -> Result<()> {
        self.0.set_color(
            ColorSpec::new()
                .set_bold(true)
                .set_fg(Some(Color::Ansi256(255))),
        )
    }

    fn label_color(&mut self) -> Result<()> {
        self.0.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
    }

    fn code_color(&mut self) -> Result<()> {
        self.0
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
    }

    fn primary_color(&mut self) -> Result<()> {
        self.0.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
    }

    fn write_level(&mut self, level: Level) -> Result<()> {
        match level {
            Level::Bug => {
                self.bug_color()?;

                write!(&mut self.0, "  bug")?;
            }
            Level::Error => {
                self.error_color()?;

                write!(&mut self.0, "error")?;
            }
            Level::Warning => {
                self.warn_color()?;

                write!(&mut self.0, " warn")?;
            }
            Level::Note => {
                self.text_color()?;

                write!(&mut self.0, " note")?;
            }
            Level::Help => {
                self.help_color()?;

                write!(&mut self.0, " help")?;
            }
        }

        Ok(())
    }

    fn write_code(&mut self, code: usize) -> Result<()> {
        write!(&mut self.0, "[{:06?}]", code)
    }

    fn write_header(&mut self, diagnostic: &Diagnostic) -> Result<()> {
        self.write_level(diagnostic.level)?;

        if let Some(code) = diagnostic.code {
            self.write_code(code)?;
        }

        self.text_color()?;

        writeln!(&mut self.0, ": {}", diagnostic.message)?;

        Ok(())
    }

    fn write_notes(&mut self, prefix_width: usize, diagnostic: &Diagnostic) -> Result<()> {
        for label in &diagnostic.nodes {
            self.label_color()?;
            write!(&mut self.0, "{} =", " ".repeat(prefix_width))?;
            self.code_color()?;
            writeln!(&mut self.0, " {}", label)?;
        }

        Ok(())
    }

    fn write_snippets<F>(&mut self, files: &F, diagnostic: &Diagnostic) -> Result<()>
    where
        F: Files,
    {
        for label in &diagnostic.labels {
            let prefix_width = self.write_file_snippet(files, label)?;
            self.write_notes(prefix_width, diagnostic)?;
        }

        Ok(())
    }

    fn write_file_snippet<'a, F>(&mut self, files: &F, label: &Label<'a>) -> Result<usize>
    where
        F: Files,
    {
        let mut lines = HashSet::new();
        let mut inline_labels = HashMap::new();
        let mut multiline_lables = HashMap::new();
        let mut multilines = 0usize;

        let location = files.to_location(label.id, &label.primary.range);

        let mut max_lines = location.end.lines;

        lines.insert(location.start.lines);
        lines.insert(location.end.lines);

        if location.start.lines == location.end.lines {
            inline_labels.insert(
                location.start.lines,
                (location, &label.primary.message, true),
            );
        } else {
            multiline_lables
                .entry(location.start.lines)
                .or_insert(vec![])
                .push((location.start.cols, None, multilines));

            multiline_lables
                .entry(location.start.lines)
                .or_insert(vec![])
                .push((location.end.cols, Some(&label.primary.message), multilines));

            multilines += 1;
        }

        for region in &label.secondary {
            let location = files.to_location(label.id, &region.range);

            if location.end.lines > max_lines {
                max_lines = location.end.lines;
            }

            lines.insert(location.start.lines);
            lines.insert(location.end.lines);

            if location.start.lines == location.end.lines {
                inline_labels.insert(location.start.lines, (location, &region.message, false));
            } else {
                multiline_lables
                    .entry(location.start.lines)
                    .or_insert(vec![])
                    .push((location.start.cols, None, multilines));

                multiline_lables
                    .entry(location.end.lines)
                    .or_insert(vec![])
                    .push((location.end.cols, Some(&region.message), multilines));

                multilines += 1;
            }
        }

        let prefix_width = max_lines.to_string().len();

        let mut lines = lines.drain().collect::<Vec<_>>();

        lines.sort();

        self.label_color()?;

        writeln!(
            &mut self.0,
            "{} ┌─ {}",
            " ".repeat(prefix_width),
            files.to_file_name(label.id)
        )?;

        let ident_size = multilines + 1;

        for line in lines {
            self.label_color()?;
            write!(&mut self.0, "{:>width$} │", line, width = prefix_width)?;
            self.code_color()?;
            let line_content = files.as_str(label.id, line);
            writeln!(
                &mut self.0,
                "{}{}",
                " ".repeat(ident_size * 2),
                line_content
            )?;

            if let Some(multilines) = multiline_lables.get(&line) {
                self.label_color()?;

                for (offset, label, index) in multilines {
                    if label.is_none() {
                        write!(&mut self.0, "{} │", " ".repeat(prefix_width))?;
                        writeln!(
                            &mut self.0,
                            "{}╭{}'",
                            " ".repeat(*index * 2 + 1),
                            "─".repeat(*offset + ident_size - *index * 2 - 4)
                        )?;
                    }
                }
            }

            if let Some((location, message, primary)) = inline_labels.get(&line) {
                self.label_color()?;
                write!(&mut self.0, "{} │", " ".repeat(prefix_width))?;

                let prefix = UnicodeWidthStr::width(&line_content[..location.start.cols - 1]);

                let content = UnicodeWidthStr::width(
                    &line_content[location.start.cols - 1..location.end.cols - 1],
                );

                write!(&mut self.0, "{}", " ".repeat(prefix + ident_size))?;

                if *primary {
                    self.primary_color()?;
                    write!(&mut self.0, "{}", "^".repeat(content))?;
                } else {
                    write!(&mut self.0, "{}", "-".repeat(content))?;
                }

                writeln!(&mut self.0, " {}", message)?;
            }

            if let Some(multilines) = multiline_lables.get(&line) {
                self.label_color()?;

                for (offset, label, index) in multilines {
                    if let Some(label) = label {
                        write!(&mut self.0, "{} │", " ".repeat(prefix_width))?;
                        writeln!(
                            &mut self.0,
                            "{}╰{}^ {}",
                            " ".repeat(*index * 2 + 1),
                            "─".repeat(*offset + ident_size - *index * 2 - 4),
                            label
                        )?;
                    }
                }
            }
        }

        Ok(prefix_width)
    }
}

impl Renderer for Term {
    type Error = std::io::Error;

    fn render<'a, F, D>(&mut self, files: &F, diagnostic: D) -> Result<()>
    where
        F: Files,
        crate::Diagnostic<'a>: From<D>,
    {
        let diagnostic: Diagnostic<'a> = diagnostic.into();

        self.write_header(&diagnostic)?;

        self.write_snippets(files, &diagnostic)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Diagnostic, Label, Renderer, SourceCodes};

    use super::Term;

    #[test]
    fn test_term() {
        let mut term = Term::default();

        let mut files = SourceCodes::default();

        files.add(
            "FizzBuzz,fun",
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
        );

        term.render(
            &files,
            Diagnostic::bug("`case` clauses have incompatible types")
                .with_code(10)
                .with_label(
                    Label::new(0, 328..331, "expected `String`, found `Nat`")
                        .with_secondary(211..331, "`case` clauses have incompatible types")
                        .with_secondary(258..268, "expected type `String` found here")
                        .with_secondary(258..331, "this is found to be of type `String`")
                        .with_secondary(284..290, "this is found to be of type `String`")
                        .with_secondary(306..312, "this is found to be of type `String`")
                        .with_secondary(186..192, "expected type `String` found here"),
                )
                .with_note(unindent::unindent(
                    "
                        expected type `String`
                                found type `Nat`
                    ",
                )),
        )
        .unwrap();
    }
}
