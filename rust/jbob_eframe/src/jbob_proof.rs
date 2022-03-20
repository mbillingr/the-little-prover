use crate::sexpr_editor::SexprEditor;
use crate::sexpr_view::SexprView;
use eframe::egui;
use jbob_app::{proof, Sexpr};

pub struct JbobProof {
    defs: Sexpr,
    pfs_edit: SexprEditor,
    result: SexprView,
}

impl JbobProof {
    pub fn new(defs: impl Into<Sexpr>) -> Self {
        JbobProof {
            defs: defs.into(),
            pfs_edit: SexprEditor::new(),
            result: SexprView::new(Sexpr::Stat("<pending>")),
        }
    }
}

impl egui::Widget for &mut JbobProof {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let r = ui.add(&mut self.pfs_edit);

            if r.changed() {
                self.result
                    .set_expr(proof(&self.defs, self.pfs_edit.expr()));
            }

            ui.add(&mut self.result);
        });

        // todo: how can we return a Response without creating another widget?
        ui.label("")
    }
}
