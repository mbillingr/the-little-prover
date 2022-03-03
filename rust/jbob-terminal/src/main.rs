use crossterm::event::{poll, KeyEvent};
use crossterm::event::{read, Event, KeyCode};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute, queue, style, style::ContentStyle, terminal};
use jbob_app::items::{EventHandler, Framed, Item, SexprView};
use jbob_app::{RenderTarget, Style, TextBuffer};
use std::io::stdout;
use std::io::{Result, Stdout, Write};
use std::time::Duration;

use jbob::j_bob;
use jbob::jbob_runtime::Context;

fn main() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (w, h) = terminal::size()?;
    let mut buffer: TextBuffer = TextBuffer::new(w as usize, h as usize);

    let ctx = Context::new();

    let exp = j_bob::axioms(&ctx).into();

    let mut sxv = SexprView::new(exp, 40, 20);

    'main_loop: loop {
        buffer.clear('╳', Style::Background);

        Framed::new(sxv.clone()).draw(&mut buffer, 2, 1);

        buffer.render(&mut Output::new(&mut stdout))?;

        while poll(Duration::from_micros(0))? {
            let event = read()?;
            if !sxv.handle_event(&adapt_event(event)) {
                match event {
                    Event::Resize(w, h) => {
                        buffer.resize(w as usize, h as usize);
                        sxv.resize(w as usize - 7, h as usize - 5)
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => break 'main_loop,
                    _ => {}
                }
            }
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen,)?;
    disable_raw_mode()?;

    Ok(())
}

struct Output<'a> {
    stdout: &'a mut Stdout,
    current_style: style::ContentStyle,
}

impl<'a> Output<'a> {
    pub fn new(stdout: &'a mut Stdout) -> Self {
        Output {
            stdout,
            current_style: Default::default(),
        }
    }
}

impl RenderTarget for Output<'_> {
    type Error = std::io::Error;

    fn prepare(&mut self) -> Result<()> {
        queue!(self.stdout, cursor::MoveTo(0, 0))
    }

    fn finalize(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    fn write_char(&mut self, ch: char, s: jbob_app::Style) -> Result<()> {
        let s = adapt_style(s);

        if s != self.current_style {
            queue!(self.stdout, style::SetAttribute(style::Attribute::Reset))?;

            if s.background_color.is_some() {
                queue!(
                    self.stdout,
                    style::SetBackgroundColor(s.background_color.unwrap())
                )?;
            }

            if s.foreground_color.is_some() {
                queue!(
                    self.stdout,
                    style::SetForegroundColor(s.foreground_color.unwrap())
                )?;
            }

            queue!(self.stdout, style::SetAttributes(s.attributes))?;

            self.current_style = s;
        }

        write!(self.stdout, "{}", ch)
    }
}

fn adapt_style(s: jbob_app::Style) -> style::ContentStyle {
    use jbob_app::Style::*;
    match s {
        Default => ContentStyle::new().white().on_dark_grey().bold(),
        Background => ContentStyle::new().dark_green().on_dark_grey(),
        Frame => ContentStyle::new().black().on_dark_grey(),
        Highlight => ContentStyle::new().black().on_dark_green(),
    }
}

pub fn adapt_event(e: crossterm::event::Event) -> jbob_app::Event {
    use crossterm::event::Event as X;
    use crossterm::event::KeyCode::*;
    use jbob_app::Event as Y;
    match e {
        X::Key(KeyEvent { code: Char(ch), .. }) => Y::Edit(ch),
        X::Key(KeyEvent {
            code: Backspace, ..
        }) => Y::EditBackspace,
        X::Key(KeyEvent { code: Delete, .. }) => Y::EditDelete,
        X::Key(KeyEvent { code: Left, .. }) => Y::NavLeft,
        X::Key(KeyEvent { code: PageDown, .. }) => Y::EditWrap,
        X::Key(KeyEvent { code: PageUp, .. }) => Y::EditUnwrap,
        X::Key(KeyEvent { code: Right, .. }) => Y::NavRight,
        X::Key(KeyEvent { code: Up, .. }) => Y::NavUp,
        X::Key(KeyEvent { code: Down, .. }) => Y::NavDown,
        _ => Y::Unknown,
    }
}
