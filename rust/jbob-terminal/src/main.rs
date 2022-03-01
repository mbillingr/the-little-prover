use crossterm::event::KeyEvent;
use crossterm::event::{read, Event, KeyCode};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute, queue, style, style::ContentStyle, terminal};
use jbob_app::items::{Framed, Item, EventHandler, SexprView};
use jbob_app::{PrettyExpr, RenderTarget, Style, TextBuffer};
use std::io::stdout;
use std::io::{Result, Stdout, Write};

fn main() -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (w, h) = terminal::size()?;
    let mut buffer: TextBuffer = TextBuffer::new(w as usize, h as usize);

    let exp = PrettyExpr::empty_list();

    let mut sxv = SexprView::new(exp, 25, 10);

    loop {
        buffer.clear('╳', Style::Background);

        Framed::new(sxv.clone()).draw(&mut buffer, 2, 1);

        buffer.render(&mut Output(&mut stdout))?;

        let event = read()?;
        if !sxv.handle_event(&adapt_event(event)) {
            match event {
                Event::Resize(w, h) => {
                    buffer.resize(w as usize, h as usize);
                    sxv.resize(w as usize - 7, h as usize - 5)
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => break,
                _ => {}
            }
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen,)?;
    disable_raw_mode()?;

    Ok(())
}

struct Output<'a>(&'a mut Stdout);

impl RenderTarget for Output<'_> {
    type Error = std::io::Error;

    fn prepare(&mut self) -> Result<()> {
        queue!(self.0, cursor::MoveTo(0, 0))
    }

    fn finalize(&mut self) -> Result<()> {
        self.0.flush()
    }

    fn write_char(&mut self, ch: char, s: jbob_app::Style) -> Result<()> {
        let s = adapt_style(s);
        queue!(self.0, style::PrintStyledContent(s.apply(ch)))
    }
}

fn adapt_style(s: jbob_app::Style) -> style::ContentStyle {
    use jbob_app::Style::*;
    match s {
        Default => ContentStyle::new().white().on_dark_grey(),
        Background => ContentStyle::new().dark_green().on_dark_grey().bold(),
        Frame => ContentStyle::new().black().on_dark_grey(),
        Highlight => ContentStyle::new().black().on_dark_green(),
    }
}

pub fn adapt_event(e: crossterm::event::Event) -> jbob_app::Event {
    use jbob_app::Event as Y;
    use crossterm::event::Event as X;
    use crossterm::event::KeyCode::*;
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
