use crate::styles::Style;
use crate::sxfmt::PrettyExpr;

#[derive(Clone)]
pub struct SexprEditor {
    expr: PrettyExpr<Style>,
    cursor: Vec<usize>,
}

impl SexprEditor {
    pub fn new(expr: PrettyExpr<Style>) -> Self {
        SexprEditor {
            expr,
            cursor: vec![],
        }
    }

    pub fn expr(&self) -> &PrettyExpr<Style> {
        &self.expr
    }

    pub fn cursor(&self) -> &[usize] {
        &self.cursor
    }

    fn try_push_cursor(&mut self) -> bool {
        self.cursor.push(0);
        if self.expr.is_valid_path(&self.cursor) {
            true
        } else {
            self.cursor.pop().unwrap();
            false
        }
    }

    pub fn move_cursor_out_of_list(&mut self) {
        self.cursor.pop();
    }

    pub fn move_cursor_into_list(&mut self) {
        self.try_push_cursor();
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

    pub fn move_cursor_next(&mut self) {
        if !self.try_push_cursor() {
            self.advance_cursor()
        }
    }

    pub fn advance_cursor(&mut self) {
        while !self.cursor.is_empty() {
            *self.cursor.last_mut().unwrap() += 1;

            if self.expr.is_valid_path(&self.cursor) {
                return;
            } else {
                self.cursor.pop();
            }
        }
    }

    pub fn move_cursor_prev(&mut self) {
        match self.cursor.last_mut() {
            None => self.cursor = self.expr.path_to_last_element().into(),
            Some(0) => {
                self.cursor.pop();
            }
            Some(c) => {
                *c -= 1;
                let p = self.expr.get(self.cursor()).unwrap().path_to_last_element();
                self.cursor.extend(p);
            }
        }
    }

    pub fn unadvance_cursor(&mut self) {
        match self.cursor.last_mut() {
            Some(0) => {
                self.cursor.pop();
            }
            Some(c) => {
                *c -= 1;
            }
            _ => {}
        }
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
