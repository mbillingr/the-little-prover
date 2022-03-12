use eframe::egui;
use jbob::jbob_runtime;
use jbob_app::{Formatter, PrettyExpr, PrettyFormatter};
use std::sync::Arc;

pub struct SexprView<'a> {
    expr: jbob_runtime::S<'a>,
    cached_layout: Option<((jbob_runtime::S<'a>, f32), Arc<egui::Galley>)>,
}

impl<'a> SexprView<'a> {
    pub fn new(expr: jbob_runtime::S<'a>) -> Self {
        SexprView {
            expr,
            cached_layout: None,
        }
    }
}

impl<'a> egui::Widget for &mut SexprView<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut s = String::new();
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut s)
                    .desired_width(500.0)
                    .code_editor()
                    .layouter(&mut |ui, _, w| {
                        if let Some(((c_exp, c_w), cache)) = self.cached_layout.as_ref() {
                            if *c_exp == self.expr && *c_w == w {
                                return cache.clone();
                            }
                        }
                        let cache = sexpr_layouter(ui, self.expr.into(), w);
                        self.cached_layout = Some(((self.expr, w), cache.clone()));
                        cache
                    }),
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

    println!("laying out...");

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
