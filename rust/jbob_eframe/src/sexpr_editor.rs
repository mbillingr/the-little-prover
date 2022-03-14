use eframe::egui;
use jbob_app::{Formatter, PrettyExpr, PrettyFormatter};
use std::sync::Arc;
use eframe::egui::{Key, Modifiers};
use crate::sexpr_layout::SexprLayout;

pub struct SexprEditor {
    editor: jbob_app::sexpr_editor::SexprEditor,
    layout: SexprLayout,
}

impl SexprEditor {
    pub fn new() -> Self {
        SexprEditor {
            editor: jbob_app::sexpr_editor::SexprEditor::new(PrettyExpr::Stat("HELLO")),
            layout: SexprLayout::new(),
        }
    }
}

impl egui::Widget for &mut SexprEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        {
            let mut input = ui.input_mut();
            if input.consume_key(Modifiers::default(), Key::S) {
                self.editor.append_at_cursor("s");
                self.layout.clear();
            }
        }

        let mut s = String::new();
        ui.add(
            egui::TextEdit::multiline(&mut s)
                .desired_width(500.0)
                .code_editor()
                .layouter(&mut |ui, _, w| self.layout.compute_once(ui, self.editor.expr(), w)),
        )
    }
}
