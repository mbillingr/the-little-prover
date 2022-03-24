use crate::sexpr_layout::build_sexpr_ui;
use eframe::egui;
use jbob_glue::Sexpr;

pub struct SexprView {
    expr: Sexpr,
}

impl SexprView {
    pub fn new(expr: impl Into<Sexpr>) -> Self {
        SexprView { expr: expr.into() }
    }

    pub fn expr(&self) -> &Sexpr {
        &self.expr
    }

    pub fn set_expr(&mut self, expr: impl Into<Sexpr>) {
        self.expr = expr.into();
    }
}

impl egui::Widget for &mut SexprView {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        build_sexpr_ui(
            ui,
            self.expr.clone(),
            egui::FontId::monospace(14.0),
            ui.max_rect().width(),
        );

        // todo: how can we return a Response without creating another widget?
        ui.label("")
    }
}
