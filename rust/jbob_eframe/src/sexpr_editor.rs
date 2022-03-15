use crate::sexpr_layout::SexprLayout;
use eframe::egui;
use eframe::egui::{Event, Key, Modifiers};
use jbob_app::PrettyExpr;

pub struct SexprEditor {
    editor: jbob_app::sexpr_editor::SexprEditor,
    layout: SexprLayout,
}

impl SexprEditor {
    pub fn new() -> Self {
        SexprEditor {
            //editor: jbob_app::sexpr_editor::SexprEditor::new(PrettyExpr::list(vec![PrettyExpr::Stat("HELLO"), PrettyExpr::Stat("WORLD")])),
            editor: jbob_app::sexpr_editor::SexprEditor::new(PrettyExpr::Stat("HELLO")),
            layout: SexprLayout::new(),
        }
    }
}

impl egui::Widget for &mut SexprEditor {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut s = String::new();
        let mut output = egui::TextEdit::multiline(&mut s)
            .desired_width(500.0)
            .interactive(false)
            .layouter(&mut |ui, _, w| self.layout.compute_once(ui, self.editor.expr(), w))
            .show(ui);
        let response = output.response;

        /*let lbl = egui::Button::new(self.layout.compute_once(ui, self.editor.expr(), 250.0));
        let response = ui.add(lbl);*/

        if response.has_focus() {
            let mut input = ui.input_mut();

            for event in &input.events {
                match event {
                    Event::Text(s) => {
                        self.editor.append_at_cursor(s);
                        self.layout.clear();
                    }
                    _ => println!("{:?}", event),
                }
            }
        }

        response
    }
}
