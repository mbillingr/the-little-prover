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
        let mut expr = self.editor.expr().clone();

        if ui.memory().has_focus(ui.id()) {
            let mut input = ui.input_mut();

            for event in &input.events {
                match event {
                    Event::Key {
                        key: Key::ArrowRight,
                        pressed: true,
                        ..
                    } => {
                        self.editor.move_cursor_next();
                        self.layout.clear();
                    }
                    Event::Key {
                        key: Key::ArrowLeft,
                        pressed: true,
                        ..
                    } => {
                        self.editor.move_cursor_prev();
                        self.layout.clear();
                    }
                    Event::Text(s) => {
                        self.editor.append_at_cursor(s);
                        self.layout.clear();
                    }
                    _ => println!("{:?}", event),
                }
                println!("{:?}", self.editor.cursor());
            }

            // abuse expr styling for highlighting the cursor position
            expr = expr
                .with_style(self.editor.cursor(), Style::Highlight)
                .unwrap();
        }

        build_sexpr_ui(
            ui,
            expr,
            egui::FontId::monospace(14.0),
            ui.max_rect().width(),
        );

        ui.interact(ui.min_rect(), ui.id(), Sense::click())
    }
}
