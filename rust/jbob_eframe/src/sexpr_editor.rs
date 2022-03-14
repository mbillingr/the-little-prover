use eframe::egui;
use jbob_app::{Formatter, PrettyExpr, PrettyFormatter};
use std::sync::Arc;

pub struct SexprEditor {
    editor: jbob_app::sexpr_editor::SexprEditor,
    cached_layout: Option<(f32, Arc<egui::Galley>)>,
}

impl SexprEditor {
    pub fn new() -> Self {
        SexprEditor {
            editor: jbob_app::sexpr_editor::SexprEditor::new(PrettyExpr::Stat("HELLO")),
            cached_layout: None,
        }
    }
}

impl egui::Widget for &mut SexprEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        println!("{:?}", ui.input().keys_down);
        ui.label("hello")
    }
}
