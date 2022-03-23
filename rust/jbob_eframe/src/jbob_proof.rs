use crate::sexpr_editor::SexprEditor;
use crate::sexpr_path::SexprPathSelector;
use crate::sexpr_view::SexprView;
use eframe::egui;
use jbob_app::{define, proof, Sexpr};

const ID_OFFSET: usize = 10;

struct ProofStep {
    partial_result: SexprPathSelector,
    rewrite: SexprEditor,
}

pub struct JbobProof {
    defs: Sexpr,
    statement: SexprEditor,
    seed: SexprEditor,
    steps: Vec<ProofStep>,
    resulting_defs: Option<(Sexpr, Sexpr)>,
}

impl JbobProof {
    pub fn new(defs: impl Into<Sexpr>) -> Self {
        JbobProof {
            defs: defs.into(),
            statement: SexprEditor::new(1, Sexpr::empty_list()),
            seed: SexprEditor::new(2, Sexpr::Stat("nil")),
            steps: vec![],
            resulting_defs: None,
        }
    }

    pub fn take_resulting_defs(&mut self) -> Option<(Sexpr, Sexpr)> {
        self.resulting_defs.take()
    }
}

impl egui::Widget for &mut JbobProof {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut changed = ui.horizontal(|ui| {
                ui.label("Claim: ");
                ui.add(&mut self.statement).changed()
            }).inner;
            changed |= ui.horizontal(|ui| {
                ui.label("Seed: ");
                ui.add(&mut self.seed).changed()
            }).inner;

            let mut extendable = true;
            let mut success = false;
            let mut proof_steps = vec![];
            for step in &mut self.steps {
                if changed {
                    let result = proof(
                        &self.defs,
                        self.statement.expr(),
                        self.seed.expr(),
                        &proof_steps,
                    );
                    step.partial_result.set_expr(result);
                }
                ui.label("----------------------");
                if let Sexpr::Quote(q) = step.partial_result.expr() {
                    ui.add(&mut SexprView::new(step.partial_result.expr().clone()));
                    extendable = false;
                    success = q.get_text() == Some("t");
                    break;
                } else {
                    ui.horizontal(|ui| {
                        changed |= ui.add(&mut step.partial_result).changed();
                        if ui.button("copy").clicked() {
                            step.rewrite.set_expr(step.partial_result.selection().clone());
                            changed = true;
                        }
                    });
                    changed |= ui.add(&mut step.rewrite).changed();
                }
                let path = step.partial_result.path_expr();
                proof_steps.push((path.clone(), step.rewrite.expr().clone()));
            }

            if extendable {
                let id = self.steps.len() * 2 + ID_OFFSET;
                let result = proof(
                    &self.defs,
                    self.statement.expr(),
                    self.seed.expr(),
                    &proof_steps,
                );
                self.steps.push(ProofStep {
                    partial_result: SexprPathSelector::new(id + 0, result),
                    rewrite: SexprEditor::new(id + 1, Sexpr::empty_list()),
                })
            }

            if success {
                if ui.button("define this").clicked() {
                    let result = define(
                        &self.defs,
                        self.statement.expr(),
                        self.seed.expr(),
                        &proof_steps,
                    );
                    self.resulting_defs = Some(result);
                }
            }
        });

        // todo: how can we return a Response without creating another widget?
        ui.label("")
    }
}
