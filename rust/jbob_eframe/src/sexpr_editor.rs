use crate::sexpr_layout::{build_sexpr_ui, SexprLayout};
use eframe::egui;
use eframe::egui::{Event, Key, Modifiers, Sense};
use jbob_app::{PrettyExpr, Style};

pub struct SexprEditor {
    editor: jbob_app::sexpr_editor::SexprEditor,
    layout: SexprLayout,
}

impl SexprEditor {
    pub fn new() -> Self {
        SexprEditor {
            editor: jbob_app::sexpr_editor::SexprEditor::new(PrettyExpr::list(vec![
                PrettyExpr::Stat("defun"),
                PrettyExpr::quote(PrettyExpr::Stat("WORLD")),
                PrettyExpr::Stat("HELLO"),
                PrettyExpr::quote(PrettyExpr::Stat("WORLD")),
            ])),
            //editor: jbob_app::sexpr_editor::SexprEditor::new(PrettyExpr::Stat("HELLO")),
            layout: SexprLayout::new(),
        }
    }
}

impl egui::Widget for &mut SexprEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let expr = if ui.memory().has_focus(ui.id()) {
            let mut input = ui.input_mut();

            if !input.events.is_empty() {
                self.layout.clear();
            }

            for event in &input.events {
                self.editor.handle_event(&adapt_event(event));
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

        let response = ui.interact(rect, ui.id(), Sense::click());

        if response.clicked() {
            ui.memory().request_focus(ui.id());
        }

        response
    }
}

pub fn adapt_event(e: &egui::Event) -> jbob_app::Event {
    use egui::Event as X;
    use jbob_app::Event as Y;
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
