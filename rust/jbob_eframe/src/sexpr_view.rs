use crate::sexpr_layout::{build_sexpr_ui, SexprLayout};
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
            build_sexpr_ui(
                ui,
                self.expr.clone(),
                egui::FontId::monospace(14.0),
                ui.max_rect().width(),
            );
        });

        // todo: how can we return a Response without creating another widget?
        ui.label("")
    }
}
