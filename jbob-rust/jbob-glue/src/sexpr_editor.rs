use crate::styles::Style;
use crate::sxfmt::PrettyExpr;
use crate::Event;

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

    pub fn set_expr(&mut self, expr: PrettyExpr<Style>) {
        self.expr = expr;
        if !self.expr.is_valid_path(self.cursor()) {
            self.cursor = vec![];
        }
    }

    pub fn cursor(&self) -> &[usize] {
        &self.cursor
    }

    pub fn selection(&self) -> &PrettyExpr<Style> {
        self.expr().get(self.cursor()).unwrap()
    }

    pub fn replace_selection(&mut self, expr: PrettyExpr<Style>) -> PrettyExpr<Style> {
        let path = self.cursor().to_vec();
        std::mem::replace(self.expr.get_mut(&path).unwrap(), expr)
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
            x.as_vec_mut()
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
                    let elements = x.as_vec_mut().unwrap();
                    elements.insert(c_elem + 1, PrettyExpr::empty_list());
                    self.move_cursor_in_list(1);
                }
            }
            _ => {}
        }
    }

    pub fn insert_element_before_cursor(&mut self) {
        match self.cursor.as_slice() {
            [c_list @ .., c_elem] => {
                let c_elem = *c_elem;
                let x = self.expr.get_mut(c_list).unwrap();
                if x.is_quotation() {
                    self.move_cursor_out_of_list();
                    self.insert_element_before_cursor();
                } else {
                    let elements = x.as_vec_mut().unwrap();
                    elements.insert(c_elem, PrettyExpr::empty_list());
                }
            }
            _ => {}
        }
    }

    pub fn grow_list_left(&mut self) {
        match self.cursor.as_slice() {
            [c_outer @ .., c_elem] if *c_elem > 0 => {
                if self.selection().is_list() {
                    let outer = self.expr.get_mut(c_outer).unwrap();
                    let idx_before = c_elem - 1;
                    let moved_item = outer.remove(&[idx_before]).unwrap();
                    let inner = outer.get_mut(&[idx_before]).unwrap();
                    inner.as_vec_mut().unwrap().insert(0, moved_item);
                    *self.cursor.last_mut().unwrap() = idx_before;
                }
            }
            _ => {}
        }
    }

    pub fn grow_list_right(&mut self) {
        match self.cursor.as_slice() {
            [c_outer @ .., c_elem] => {
                if self.selection().is_list() {
                    let outer = self.expr.get_mut(c_outer).unwrap();
                    let idx_next = c_elem + 1;
                    if idx_next < outer.len() {
                        let moved_item = outer.remove(&[idx_next]).unwrap();
                        let inner = outer.get_mut(&[*c_elem]).unwrap();
                        inner.as_vec_mut().unwrap().push(moved_item);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn shrink_list_left(&mut self) {
        match self.cursor.as_slice() {
            [c_outer @ .., c_list] => {
                if !self.selection().is_atom() {
                    let outer = self.expr.get_mut(c_outer).unwrap().as_vec_mut().unwrap();
                    let moved_item = outer[*c_list].as_vec_mut().unwrap().remove(0);
                    outer.insert(*c_list, moved_item);
                    *self.cursor.last_mut().unwrap() += 1;
                }
            }
            _ => {}
        }
    }

    pub fn shrink_list_right(&mut self) {
        match self.cursor.as_slice() {
            [c_outer @ .., c_list] => {
                if !self.selection().is_atom() {
                    let outer = self.expr.get_mut(c_outer).unwrap().as_vec_mut().unwrap();
                    let moved_item = outer[*c_list].as_vec_mut().unwrap().pop().unwrap();
                    outer.insert(c_list + 1, moved_item);
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
        if let Some([y]) = x.as_slice() {
            *x = y.clone();
        } else if let Some(y) = x.quoted_value() {
            *x = y.clone();
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        use Event::*;
        match event {
            NavNext => self.move_cursor_next(),
            NavPrev => self.move_cursor_prev(),
            NavNextFast => self.advance_cursor(),
            NavPrevFast => self.unadvance_cursor(),
            EditWrap => self.wrap_cursor_in_list(),
            EditUnwrap => self.unwrap_unary_list_at_cursor(),
            EditDelete => self.delete_cursor_element(),
            EditInsert => self.insert_element_before_cursor(),
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
            EditGrowLeft => self.grow_list_left(),
            EditGrowRight => self.grow_list_right(),
            EditShrinkLeft => self.shrink_list_left(),
            EditShrinkRight => self.shrink_list_right(),
            _ => return false,
        }
        true
    }
}
