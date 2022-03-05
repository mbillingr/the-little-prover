use crate::events::Event;
use crate::widgets::{EventHandler, Widget};
use crate::styles::Style;
use crate::sxfmt::{Formatter, PrettyExpr, PrettyFormatter};
use crate::textbuffer::TextBuffer;

#[derive(Clone)]
pub struct SexprView {
    expr: PrettyExpr<Style>,
    cursor: Vec<usize>,
}

impl SexprView {
    pub fn new(expr: PrettyExpr<Style>) -> Self {
        SexprView {
            expr,
            cursor: vec![],
        }
    }

    pub fn move_cursor_out_of_list(&mut self) {
        self.cursor.pop();
    }

    pub fn move_cursor_into_list(&mut self) {
        self.cursor.push(0);
        if !self.expr.is_valid_path(&self.cursor) {
            self.cursor.pop().unwrap();
        }
    }

    pub fn move_cursor_in_list(&mut self, dir: i8) {
        if self.cursor.is_empty() {
            return;
        }
        let new_pos = self.cursor.pop().unwrap() as isize + dir as isize;
        let l = self.expr.get(&self.cursor).unwrap().len() as isize;
        let new_pos = (new_pos + l) % l as isize;
        self.cursor.push(new_pos as usize);
    }

    pub fn append_at_cursor(&mut self, postfix: &str) {
        let x = self.expr.get_mut(&self.cursor).unwrap();
        if let Some(text) = x.get_text() {
            let text = text.to_string() + postfix;
            *x = PrettyExpr::Atom(text);
        } else if x.is_empty_list() {
            x.elements_mut()
                .unwrap()
                .push(PrettyExpr::Atom(postfix.to_string()));
            self.move_cursor_into_list();
        }
    }

    pub fn delete_at_cursor(&mut self) {
        let x = self.expr.get_mut(&self.cursor).unwrap();
        if let Some(text) = x.get_text() {
            let mut text = text.to_string();
            text.pop();
            if text.is_empty() {
                *x = PrettyExpr::list(vec![]);
            } else {
                *x = PrettyExpr::Atom(text);
            }
        }
    }

    pub fn delete_cursor_element(&mut self) {
        match self.cursor.as_slice() {
            [c_list @ .., c_elem] => {
                let c_elem = *c_elem;
                let x = self.expr.get_mut(c_list).unwrap();
                x.remove_item(c_elem);
                if x.is_empty_list() {
                    self.cursor.pop();
                } else {
                    let last = self.cursor.last_mut().unwrap();
                    *last = usize::min(c_elem, x.len() - 1)
                }
            }
            [] => {} //self.expr = PrettyExpr::empty_list(),
        }
    }

    pub fn insert_element_after_cursor(&mut self) {
        match self.cursor.as_slice() {
            [c_list @ .., c_elem] => {
                let c_elem = *c_elem;
                let x = self.expr.get_mut(c_list).unwrap();
                if x.is_quotation() {
                    self.move_cursor_out_of_list();
                    self.insert_element_after_cursor();
                } else {
                    let elements = x.elements_mut().unwrap();
                    elements.insert(c_elem + 1, PrettyExpr::empty_list());
                    self.move_cursor_in_list(1);
                }
            }
            _ => {}
        }
    }

    pub fn quote_cursor(&mut self) {
        let x = self.expr.get_mut(&self.cursor).unwrap();
        let y = x.clone();
        *x = PrettyExpr::quote(y);
    }

    pub fn wrap_cursor_in_list(&mut self) {
        let x = self.expr.get_mut(&self.cursor).unwrap();
        let y = x.clone();
        *x = PrettyExpr::list(vec![y]);
    }

    pub fn unwrap_unary_list_at_cursor(&mut self) {
        let x = self.expr.get_mut(&self.cursor).unwrap();
        if let Some([y]) = x.elements() {
            *x = y.clone();
        } else if let Some(y) = x.quoted_value() {
            *x = y.clone();
        }
    }
}

impl Widget for SexprView {
    fn draw(&self, buf: &mut TextBuffer, x: usize, y: usize, width: usize, height: usize) {
        let mut pf = PrettyFormatter::default();
        pf.max_code_width = width;
        let mut pe = pf.pretty(self.expr.clone());

        pe = pe
            .with_style(&[], Style::Default)
            .unwrap()
            .with_style(&self.cursor, Style::Highlight)
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
            NavLeft => self.move_cursor_out_of_list(),
            NavRight => self.move_cursor_into_list(),
            NavDown => self.move_cursor_in_list(1),
            NavUp => self.move_cursor_in_list(-1),
            EditWrap => self.wrap_cursor_in_list(),
            EditUnwrap => self.unwrap_unary_list_at_cursor(),
            EditDelete => self.delete_cursor_element(),
            Edit('\'') => {
                self.quote_cursor();
                self.move_cursor_into_list();
            }
            Edit('(') => {
                self.wrap_cursor_in_list();
                self.move_cursor_into_list();
            }
            Edit(')') => self.move_cursor_out_of_list(),
            Edit(' ') => self.insert_element_after_cursor(),
            Edit(ch) => self.append_at_cursor(&ch.to_string()),
            EditBackspace => self.delete_at_cursor(),
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
