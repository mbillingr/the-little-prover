use eframe::egui;

pub struct SexprView {}

impl egui::Widget for &mut SexprView {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("hello")
    }
}
