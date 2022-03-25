use crate::sexpr_layout::build_sexpr_ui;
use eframe::egui;
use eframe::egui::{Event, Key, Sense};
use jbob_glue::{Sexpr, Style};

pub struct SexprEditor {
    id: usize,
    editor: jbob_glue::sexpr_editor::SexprEditor,
}

impl SexprEditor {
    pub fn new(id: usize, expr: impl Into<Sexpr>) -> Self {
        SexprEditor {
            id,
            editor: jbob_glue::sexpr_editor::SexprEditor::new(expr.into()),
        }
    }

    pub fn expr(&self) -> &Sexpr {
        self.editor.expr()
    }

    pub fn set_expr(&mut self, expr: impl Into<Sexpr>) {
        self.editor.set_expr(expr.into())
    }

    pub fn replace_cursor(&mut self, expr: impl Into<Sexpr>) {
        self.editor.replace_selection(expr.into());
    }
}

impl egui::Widget for &mut SexprEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = ui.id().with(self.id);
        let mut changed = false;
        let expr = if ui.memory().has_focus(id) {
            let input = ui.input_mut();

            for event in &input.events {
                changed |= self.editor.handle_event(&adapt_event(event));
            }

            // abuse expr styling for highlighting the cursor position
            self.editor
                .expr()
                .clone()
                .with_style(self.editor.cursor(), Style::Highlight)
                .unwrap()
        } else {
            self.editor.expr().clone()
        };

        let rect = build_sexpr_ui(
            ui,
            expr,
            egui::FontId::monospace(14.0),
            ui.max_rect().width(),
        );

        let mut response = ui.interact(rect, id, Sense::click());

        if response.clicked() {
            ui.memory().request_focus(id);
        }

        if changed {
            response.mark_changed();
        }

        response
    }
}

pub fn adapt_event(e: &egui::Event) -> jbob_glue::Event {
    use egui::Event as X;
    use jbob_glue::Event as Y;
    match e {
        X::Text(s) => Y::Edit(s.chars().next().unwrap()),
        Event::Key {
            key: Key::Backspace,
            pressed: true,
            ..
        } => Y::EditBackspace,
        Event::Key {
            key: Key::Delete,
            pressed: true,
            ..
        } => Y::EditDelete,
        Event::Key {
            key: Key::Insert,
            pressed: true,
            ..
        } => Y::EditInsert,
        Event::Key {
            key: Key::ArrowLeft,
            pressed: true,
            modifiers: egui::Modifiers { shift: true, .. },
        } => Y::EditGrowLeft,
        Event::Key {
            key: Key::ArrowRight,
            pressed: true,
            modifiers: egui::Modifiers { shift: true, .. },
        } => Y::EditGrowRight,
        Event::Key {
            key: Key::ArrowUp,
            pressed: true,
            modifiers: egui::Modifiers { shift: true, .. },
        } => Y::EditShrinkLeft,
        Event::Key {
            key: Key::ArrowDown,
            pressed: true,
            modifiers: egui::Modifiers { shift: true, .. },
        } => Y::EditShrinkRight,
        Event::Key {
            key: Key::ArrowRight | Key::ArrowDown,
            pressed: true,
            modifiers: egui::Modifiers { ctrl: true, .. },
        } => Y::NavNextFast,
        Event::Key {
            key: Key::ArrowLeft | Key::ArrowUp,
            pressed: true,
            modifiers: egui::Modifiers { ctrl: true, .. },
        } => Y::NavPrevFast,
        Event::Key {
            key: Key::ArrowRight | Key::ArrowDown,
            pressed: true,
            ..
        } => Y::NavNext,
        Event::Key {
            key: Key::ArrowLeft | Key::ArrowUp,
            pressed: true,
            ..
        } => Y::NavPrev,
        Event::Key {
            key: Key::PageDown,
            pressed: true,
            ..
        } => Y::EditWrap,
        Event::Key {
            key: Key::PageUp,
            pressed: true,
            ..
        } => Y::EditUnwrap,
        _ => Y::Unknown,
    }
}
