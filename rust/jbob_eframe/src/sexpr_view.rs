use eframe::egui;
use jbob_app::{Formatter, PrettyExpr, PrettyFormatter};
use std::sync::Arc;

pub struct SexprView {
    expr: PrettyExpr<&'static str>,
}

impl SexprView {
    pub fn new(expr: PrettyExpr<&'static str>) -> Self {
        SexprView { expr }
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
                    .layouter(&mut |ui, _, w| sexpr_layouter(ui, self.expr.clone(), w)),
            )
        });

        ui.label("hello")
    }
}

fn sexpr_layouter(
    ui: &egui::Ui,
    expr: PrettyExpr<&'static str>,
    wrap_width: f32,
) -> Arc<egui::Galley> {
    let font_id = egui::FontId::monospace(14.0);

    let char_width = ui.fonts().glyph_width(&font_id, '_');
    // assuming all chars in a monospace font have the same width
    let max_row_len = (wrap_width / char_width).floor() as usize - 1;

    let mut pf = PrettyFormatter::default();
    pf.max_code_width = max_row_len;

    let pe = pf.pretty(expr);
    let mut f = LayoutJobFormatter::new(font_id);
    pe.write(&mut f).unwrap();

    ui.fonts().layout_job(f.into())
}

struct LayoutJobFormatter {
    layout_job: egui::text::LayoutJob,
    current_style: egui::TextFormat,
}

impl LayoutJobFormatter {
    pub fn new(font_id: egui::FontId) -> Self {
        LayoutJobFormatter {
            layout_job: Default::default(),
            current_style: egui::TextFormat::simple(font_id, egui::Color32::DARK_BLUE),
        }
    }
}

impl Formatter<&'static str> for LayoutJobFormatter {
    type Error = ();
    fn write(&mut self, x: impl std::fmt::Display) -> std::result::Result<(), Self::Error> {
        self.layout_job
            .append(&x.to_string(), 0.0, self.current_style.clone());
        Ok(())
    }

    fn set_style(&mut self, _style: &&'static str) {}

    fn save_style(&mut self) {}

    fn restore_style(&mut self) {}
}

impl From<LayoutJobFormatter> for egui::text::LayoutJob {
    fn from(ljf: LayoutJobFormatter) -> Self {
        ljf.layout_job
    }
}
