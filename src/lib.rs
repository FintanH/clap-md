extern crate clap;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use clap::{App, ArgSettings};
use pulldown_cmark::{CodeBlockKind, Event, Tag};
use pulldown_cmark_to_cmark::{cmark_with_options, Options};

struct Document<'a>(Vec<Event<'a>>);

impl<'a> Document<'a> {
    fn header(&mut self, text: String, level: u32) {
        self.0.push(Event::Start(Tag::Heading(level)));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Heading(level)));
    }

    fn paragraph(&mut self, text: String) {
        self.0.push(Event::Start(Tag::Paragraph));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Paragraph));
    }
}

fn recursive(doc: &mut Document, app: &App, level: u32, skip_header: bool) {
    if !skip_header {
        doc.header(app.get_name().to_string(), level);
    }

    if let Some(about) = app.get_about() {
        doc.paragraph(about.into());
    }

    doc.paragraph(format!("Version: {}", app.render_version()));

    let mut args = app.get_arguments();
    if let Some(arg) = args.next() {
        // if !app.args.is_empty() {
        doc.paragraph("Arguments:".into());
        doc.0.push(Event::Start(Tag::List(None)));

        handle_arg(doc, &arg);
        for arg in app.get_arguments() {
            handle_arg(doc, &arg)
        }

        doc.0.push(Event::End(Tag::List(None)));
    }

    let mut subcommands = app.get_subcommands();
    let cmd = subcommands.next();

    if let Some(cmd) = cmd {
        doc.header("Subcommands".into(), level + 1);
        recursive(doc, cmd, level + 2, false);

        for cmd in subcommands {
            recursive(doc, cmd, level + 2, false);
        }
    }
}

fn handle_arg(doc: &mut Document, arg: &clap::Arg) {
    doc.0.push(Event::Start(Tag::Item));
    doc.0.push(Event::Start(Tag::Paragraph));

    doc.0
        .push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
            "text".into(),
        ))));

    let mut def = String::new();
    if let Some(short) = arg.get_short() {
        def.push_str("-");
        def.push(short);
    }
    if let Some(long) = arg.get_long() {
        if arg.get_short().is_some() {
            def.push_str("/");
        }
        def.push_str("--");
        def.push_str(long);
    }

    if arg.is_set(ArgSettings::TakesValue) {
        def.push_str("=<");
        def.push_str(arg.get_name());
        def.push_str(">");
    }

    doc.0.push(Event::Text(def.into()));
    doc.0.push(Event::Text("\n".into()));
    doc.0.push(Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(
        "text".into(),
    ))));

    let mut text = String::new();
    if let Some(help) = arg.get_help_heading() {
        if arg.get_short().is_some() || arg.get_long().is_some() {
            text.push_str(": ");
        }
        text.push_str(help);
    }
    doc.0.push(Event::Text(text.into()));

    doc.0.push(Event::End(Tag::Paragraph));
    doc.0.push(Event::End(Tag::Item));
}

/// Convert a clap App to markdown documentation
///
/// # Parameters
///
/// - `app`: A reference to a clap application definition
/// - `level`: The level for first markdown headline. If you for example want to
///     render this beneath a `## Usage` headline in your readme, you'd want to
///     set `level` to `2`.
pub fn app_to_md<'a>(app: &App<'a>, level: u32) -> Result<String, Box<dyn std::error::Error>> {
    let mut document = Document(Vec::new());
    recursive(&mut document, app, level, level > 1);
    let mut result = String::new();
    let opts = Options {
        code_block_backticks: 3,
        ..Options::default()
    };
    cmark_with_options(document.0.iter(), &mut result, None, opts)?;
    Ok(result)
}
