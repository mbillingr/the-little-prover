use crate::sexpr_layout::SexprLayout;
use eframe::egui;
use jbob_app::Sexpr;

pub struct SexprView {
    expr: Sexpr,
    layout: SexprLayout,
}

impl SexprView {
    pub fn new(expr: impl Into<Sexpr>) -> Self {
        SexprView {
            expr: expr.into(),
            layout: SexprLayout::new(),
        }
    }

    pub fn set_expr(&mut self, expr: impl Into<Sexpr>) {
        self.expr = expr.into();
        self.layout.clear();
    }
}

impl egui::Widget for &mut SexprView {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut s = String::new();
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut s)
                    .desired_width(500.0)
                    .code_editor()
                    .layouter(&mut |ui, _, w| self.layout.compute_once(ui, &self.expr, w)),
            )
        });

        // todo: how can we return a Response without creating another widget?
        ui.label("hello")
    }
}
