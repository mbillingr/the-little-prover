use crate::jbob_proof::JbobProof;
use crate::sexpr_view::SexprView;
use eframe::{egui, epi};
use jbob::{
    j_bob,
    jbob_runtime::{self, Parser},
};
use jbob_app::{sexpr_adapter, Sexpr};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp<'a> {
    jbob_context: jbob_runtime::Context<'a>,
    //jbob_defs: jbob_runtime::S<'a>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    sexpr_view: SexprView,

    proof: JbobProof,
    // this how you opt-out of serialization of a member
    //#[cfg_attr(feature = "persistence", serde(skip))]
    //value: f32,
    definitions: Sexpr,
    import_script: Option<String>,
}

impl Default for TemplateApp<'_> {
    fn default() -> Self {
        let mut jbob_context = jbob_runtime::Context::new();
        let jbob_defs = j_bob::prelude(&mut jbob_context);
        Self {
            jbob_context,
            //jbob_defs,
            sexpr_view: SexprView::new(jbob_defs),
            proof: JbobProof::new(jbob_defs),
            definitions: Sexpr::empty_list(),
            import_script: None,
        }
    }
}

impl<'a> epi::App for TemplateApp<'a> {
    fn name(&self) -> &str {
        "J/Bob GUI"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            jbob_context: _jbob_ctx,
            ..
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Import/Export").clicked() {
                        self.import_script = Some(String::new());
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                egui::warn_if_debug_build(ui);
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Definitions");
            egui::ScrollArea::vertical()
                .stick_to_bottom()
                .show(ui, |ui| {
                    ui.add(&mut self.sexpr_view);
                });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Proof");
            ui.add(&mut self.proof)
        });

        if let Some((defs, definition)) = self.proof.take_resulting_defs() {
            self.proof = JbobProof::new(defs.clone());
            self.sexpr_view.set_expr(defs);
            self.definitions.append(definition.take(&[0]).unwrap());
        }

        if let Some(script) = &mut self.import_script {
            let mut should_close = false;
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Copy/Paste a JBob script here...");
                ui.label("The \"script\" is meant to be used in");
                ui.label("(J-Bob/define (prelude) '______)");
                ui.text_edit_multiline(script);
                ui.horizontal(|ui| {
                    if ui.button("Import").clicked() {
                        let ctx = &mut jbob_runtime::Context::new();
                        match ctx.parse(script) {
                            Err(e) => {
                                ui.label(e.to_string());
                            }
                            Ok(pfs) => {
                                let defs = j_bob::j_bob_slash_define(ctx, j_bob::prelude(ctx), pfs);
                                self.sexpr_view.set_expr(defs);
                                self.definitions = pfs.into();
                                self.proof = JbobProof::new(defs.clone());
                                should_close = true;
                            }
                        }
                    }

                    if ui.button("Export").clicked() {
                        *script = self.definitions.to_string();
                    }
                })
            });
            if should_close {
                self.import_script = None;
            }
        }

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
