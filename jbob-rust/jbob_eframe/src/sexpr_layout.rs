use eframe::egui;
use jbob_glue::{Formatter, PrettyFormatter, Sexpr, Style};

pub fn build_sexpr_ui(
    ui: &mut egui::Ui,
    expr: Sexpr,
    font: egui::FontId,
    wrap_width: f32,
) -> egui::Rect {
    ui.vertical(|ui| {
        let char_width = ui.fonts().glyph_width(&font, '_');
        // assuming all chars in a monospace font have the same width
        let max_row_len = (wrap_width / char_width).floor().max(1.0) as usize - 1;

        let mut pf = PrettyFormatter::default();
        pf.max_code_width = max_row_len;

        let pe = pf.pretty(expr.highlight());
        let mut f = UiFormatter::new(ui);
        pe.write(&mut f).unwrap();
        f.write_newline().unwrap();
        f.rect
    })
    .inner
}

struct UiFormatter<'a> {
    ui: &'a mut egui::Ui,
    fragment: String,
    current_line: LineWriter,
    current_style: egui::TextFormat,
    saved_styles: Vec<egui::TextFormat>,
    rect: egui::Rect,
}

impl<'a> UiFormatter<'a> {
    pub fn new(ui: &'a mut egui::Ui) -> Self {
        UiFormatter {
            ui,
            fragment: String::new(),
            current_line: LineWriter::new(),
            current_style: egui::TextFormat::default(),
            saved_styles: vec![],
            rect: egui::Rect::NOTHING,
        }
    }

    fn new_fragment(&mut self) {
        let text = std::mem::replace(&mut self.fragment, String::new());
        let styled_text = egui::RichText::new(text)
            .monospace()
            .color(self.current_style.color)
            .background_color(self.current_style.background);
        self.current_line.append(styled_text);
    }
}

impl Formatter<Style> for UiFormatter<'_> {
    type Error = ();
    fn write(&mut self, x: impl std::fmt::Display) -> std::result::Result<(), Self::Error> {
        self.fragment.push_str(&x.to_string());
        Ok(())
    }

    fn set_style(&mut self, style: &Style) {
        self.new_fragment();
        match style {
            Style::Quote => self.current_style.color = egui::Color32::BLUE,
            Style::True => self.current_style.color = egui::Color32::from_rgb(0, 0xb7, 0),
            Style::False => self.current_style.color = egui::Color32::RED,
            Style::Keyword => {
                self.current_style.color = if self.ui.style().visuals.dark_mode {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::BLACK
                }
            }
            Style::Highlight => self.current_style.background = egui::Color32::LIGHT_BLUE,
            _ => self.current_style = egui::TextFormat::default(),
        }
    }

    fn save_style(&mut self) {
        self.saved_styles.push(self.current_style.clone())
    }

    fn restore_style(&mut self) {
        self.new_fragment();
        self.current_style = self.saved_styles.pop().unwrap();
    }

    fn write_newline(&mut self) -> std::result::Result<(), Self::Error> {
        self.new_fragment();

        let line = std::mem::replace(&mut self.current_line, LineWriter::new());
        let r = line.write(self.ui);
        self.rect = self.rect.union(r.rect);
        Ok(())
    }
}

struct LineWriter {
    parts: Vec<egui::RichText>,
}

impl LineWriter {
    fn new() -> Self {
        LineWriter { parts: vec![] }
    }

    fn append(&mut self, text: egui::RichText) {
        self.parts.push(text)
    }

    fn write(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for text in self.parts {
                ui.add(egui::Label::new(text));
            }
        })
        .response
    }
}
