use crate::events::Event;
use crate::sexpr_editor::SexprEditor;
use crate::styles::Style;
use crate::sxfmt::{Formatter, PrettyExpr, PrettyFormatter};
use crate::textbuffer::TextBuffer;
use crate::widgets::{EventHandler, Widget};

#[derive(Clone)]
pub struct SexprView(SexprEditor);

impl SexprView {
    pub fn new(expr: PrettyExpr<Style>) -> Self {
        SexprView(SexprEditor::new(expr))
    }
}

impl Widget for SexprView {
    fn draw(&self, buf: &mut TextBuffer, x: usize, y: usize, width: usize, height: usize) {
        let mut pf = PrettyFormatter::default();
        pf.max_code_width = width;
        let mut pe = pf.pretty(self.0.expr().clone());

        pe = pe
            .with_style(&[], Style::Default)
            .unwrap()
            .with_style(self.0.cursor(), Style::Highlight)
            .unwrap();

        let mut tmp = TextBuffer::new(0, 0);

        let mut cf = TextBufferFormatter::new(&mut tmp);
        pe.write(&mut cf).unwrap();

        let mut first_row = 0;

        if let Some((h0, h1)) = cf.highlight_row_range {
            if h1 >= height {
                first_row = h0 - height.saturating_sub(h1 - h0) / 2
            }
        }

        let w = width.min(tmp.width());
        let h = height.min(tmp.height() - first_row);
        buf.copy_rect(x, y, &tmp, 0, first_row, w, h);
    }
}

impl EventHandler<Event> for SexprView {
    fn handle_event(&mut self, event: &Event) -> bool {
        use Event::*;
        match event {
            NavLeft => self.0.move_cursor_out_of_list(),
            NavRight => self.0.move_cursor_into_list(),
            NavDown => self.0.move_cursor_in_list(1),
            NavUp => self.0.move_cursor_in_list(-1),
            EditWrap => self.0.wrap_cursor_in_list(),
            EditUnwrap => self.0.unwrap_unary_list_at_cursor(),
            EditDelete => self.0.delete_cursor_element(),
            Edit('\'') => {
                self.0.quote_cursor();
                self.0.move_cursor_into_list();
            }
            Edit('(') => {
                self.0.wrap_cursor_in_list();
                self.0.move_cursor_into_list();
            }
            Edit(')') => self.0.move_cursor_out_of_list(),
            Edit(' ') => self.0.insert_element_after_cursor(),
            Edit(ch) => self.0.append_at_cursor(&ch.to_string()),
            EditBackspace => self.0.delete_at_cursor(),
            _ => return false,
        }
        true
    }
}

struct TextBufferFormatter<'a> {
    buf: &'a mut TextBuffer,
    current_style: Style,
    saved_styles: Vec<Style>,
    start_column: usize,
    current_row: usize,
    cursor: (usize, usize),
    highlight_row_range: Option<(usize, usize)>,
}

impl<'a> TextBufferFormatter<'a> {
    pub fn new(buf: &'a mut TextBuffer) -> Self {
        TextBufferFormatter {
            buf,
            current_style: Default::default(),
            saved_styles: vec![],
            start_column: 0,
            current_row: 0,
            cursor: (0, 0),
            highlight_row_range: None,
        }
    }
}

impl<'a> Formatter<Style> for TextBufferFormatter<'a> {
    type Error = ();

    fn write(&mut self, x: impl std::fmt::Display) -> std::result::Result<(), Self::Error> {
        if self.cursor.1 >= self.buf.height() {
            self.buf.resize(
                self.buf.width(),
                (self.buf.height() + 1).next_power_of_two(),
            );
        }
        for ch in x.to_string().chars() {
            if self.cursor.0 >= self.buf.width() {
                self.buf.resize(
                    (self.buf.width() + 1).next_power_of_two(),
                    self.buf.height(),
                );
            }
            self.buf
                .set_char(self.cursor.0, self.cursor.1, ch, self.current_style);
            self.cursor.0 += 1;
        }
        Ok(())
    }

    fn set_style(&mut self, style: &Style) {
        self.current_style = *style;
        if *style == Style::Highlight && self.highlight_row_range.is_none() {
            self.highlight_row_range = Some((self.current_row, self.current_row));
        }
    }

    fn save_style(&mut self) {
        self.saved_styles.push(self.current_style)
    }

    fn restore_style(&mut self) {
        if self.current_style == Style::Highlight {
            self.highlight_row_range = self.highlight_row_range.map(|(s, _)| (s, self.current_row));
        }
        let style = self.saved_styles.pop().unwrap();
        self.set_style(&style);
    }

    fn write_newline(&mut self) -> std::result::Result<(), Self::Error> {
        self.current_row += 1;
        self.cursor = (self.start_column, self.current_row);
        Ok(())
    }
}
