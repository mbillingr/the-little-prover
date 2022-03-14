use eframe::egui;
use jbob_app::{Formatter, PrettyFormatter, Sexpr, Style};
use std::sync::Arc;

pub struct SexprLayout {
    cached_layout: Option<(f32, Arc<egui::Galley>)>,
}

impl SexprLayout {
    pub fn new() -> Self {
        SexprLayout {
            cached_layout: None,
        }
    }

    pub fn clear(&mut self) {
        self.cached_layout = None
    }

    pub fn compute_once(&mut self, ui: &egui::Ui, expr: &Sexpr, width: f32) -> Arc<egui::Galley> {
        if let Some((c_w, cache)) = self.cached_layout.as_ref() {
            if *c_w == width {
                return cache.clone();
            }
        }
        let layout = sexpr_layouter(ui, expr.clone(), width);
        self.cached_layout = Some((width, layout.clone()));
        layout
    }
}

fn sexpr_layouter(ui: &egui::Ui, expr: Sexpr, wrap_width: f32) -> Arc<egui::Galley> {
    let font_id = egui::FontId::monospace(14.0);

    let char_width = ui.fonts().glyph_width(&font_id, '_');
    // assuming all chars in a monospace font have the same width
    let max_row_len = (wrap_width / char_width).floor() as usize - 1;

    let mut pf = PrettyFormatter::default();
    pf.max_code_width = max_row_len;

    let pe = pf.pretty(expr.highlight());
    let mut f = LayoutJobFormatter::new(font_id);
    pe.write(&mut f).unwrap();

    ui.fonts().layout_job(f.into())
}

struct LayoutJobFormatter {
    layout_job: egui::text::LayoutJob,
    current_style: egui::TextFormat,
    saved_styles: Vec<egui::TextFormat>,
}

impl LayoutJobFormatter {
    pub fn new(font_id: egui::FontId) -> Self {
        LayoutJobFormatter {
            layout_job: Default::default(),
            current_style: egui::TextFormat::simple(font_id, egui::Color32::DARK_BLUE),
            saved_styles: vec![],
        }
    }
}

impl Formatter<Style> for LayoutJobFormatter {
    type Error = ();
    fn write(&mut self, x: impl std::fmt::Display) -> std::result::Result<(), Self::Error> {
        self.layout_job
            .append(&x.to_string(), 0.0, self.current_style.clone());
        Ok(())
    }

    fn set_style(&mut self, style: &Style) {
        match style {
            Style::Quote => self.current_style.color = egui::Color32::RED,
            Style::Keyword => self.current_style.color = egui::Color32::BLUE,
            _ => self.current_style.color = egui::Color32::DARK_BLUE,
        }
    }

    fn save_style(&mut self) {
        self.saved_styles.push(self.current_style.clone())
    }

    fn restore_style(&mut self) {
        self.current_style = self.saved_styles.pop().unwrap();
    }
}

impl From<LayoutJobFormatter> for egui::text::LayoutJob {
    fn from(ljf: LayoutJobFormatter) -> Self {
        ljf.layout_job
    }
}
