use crate::sexpr_layout::build_sexpr_ui;
use eframe::egui;
use eframe::egui::{Event, Key, Sense};
use jbob_app::{Sexpr, Style};

pub struct SexprPathSelector {
    id: usize,
    editor: jbob_app::sexpr_editor::SexprEditor,
    path_expr: Sexpr,
}

impl SexprPathSelector {
    pub fn new(id: usize, expr: impl Into<Sexpr>) -> Self {
        SexprPathSelector {
            id,
            editor: jbob_app::sexpr_editor::SexprEditor::new(expr.into()),
            path_expr: Sexpr::list(vec![]),
        }
    }

    pub fn set_expr(&mut self, expr: impl Into<Sexpr>) {
        self.editor.set_expr(expr.into());
    }

    pub fn expr(&self) -> &Sexpr {
        self.editor.expr()
    }

    pub fn selection(&self) -> &Sexpr {
        self.editor.selection()
    }

    pub fn path_expr(&self) -> &Sexpr {
        &self.path_expr
    }

    fn update_path_expr(&mut self) {
        let mut expr = self.editor.expr();
        let mut path = vec![];
        for &p in self.editor.cursor() {
            if expr.get(&[0]).and_then(|x| x.get_text()) == Some("if") {
                let s = match p {
                    1 => "Q",
                    2 => "A",
                    3 => "E",
                    _ => "?",
                };
                path.push(Sexpr::Stat(s))
            } else {
                path.push(Sexpr::Atom(format!("{}", p)));
            }
            expr = expr.get(&[p]).unwrap();
        }

        self.path_expr = Sexpr::list(path);
    }
}

impl egui::Widget for &mut SexprPathSelector {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            let id = ui.id().with(self.id);
            let mut changed = false;

            if ui.memory().has_focus(id) {
                let input = ui.input_mut();

                for event in &input.events {
                    changed |= self.editor.handle_event(&adapt_event(event));
                }
            }

            if changed {
                self.update_path_expr();
            }

            // abuse expr styling for highlighting the cursor position
            let expr = self
                .editor
                .expr()
                .clone()
                .with_style(self.editor.cursor(), Style::Highlight)
                .unwrap();

            let rect = build_sexpr_ui(
                ui,
                expr,
                egui::FontId::monospace(14.0),
                ui.max_rect().width(),
            );

            ui.label(format!("Path: {}", self.path_expr));

            let mut response = ui.interact(rect, id, Sense::click());

            if response.clicked() {
                ui.memory().request_focus(id);
            }

            if changed {
                response.mark_changed();
            }

            response
        }).inner
    }
}

pub fn adapt_event(e: &egui::Event) -> jbob_app::Event {
    use jbob_app::Event as Y;
    match e {
        Event::Key {
            key: Key::ArrowRight | Key::ArrowDown,
            pressed: true,
            modifiers: egui::Modifiers { ctrl: true, .. },
        } => Y::NavNextFast,
        Event::Key {
            key: Key::ArrowLeft | Key::ArrowUp,
            pressed: true,
            modifiers: egui::Modifiers { ctrl: true, .. },
        } => Y::NavPrevFast,
        Event::Key {
            key: Key::ArrowRight | Key::ArrowDown,
            pressed: true,
            ..
        } => Y::NavNext,
        Event::Key {
            key: Key::ArrowLeft | Key::ArrowUp,
            pressed: true,
            ..
        } => Y::NavPrev,
        _ => Y::Unknown,
    }
}
