use eframe::{egui, epi};

pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui template"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /*
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });*/

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("egui template");
            ui.hyperlink("https://github.com/emilk/egui_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/egui_template/blob/master/",
                "Source code."
            ));
            //let my_texture_id = egui::TextureId::Egui;
            //ui.image(my_texture_id, [640.0, 480.0]);

            /*
            let tex_color = Rgba::from_rgb(1.0, 0.25, 0.25);
            let vertex_color = Rgba::from_rgb(0.5, 0.75, 0.75);

            let g = egui::Gradient::one_color(Color32::from(tex_color));
            let tex = self.tex_mngr.get(*tex_allocator, &g);
            let texel_offset = 0.5 / (g.0.len() as f32);
            let uv =
                Rect::from_min_max(pos2(texel_offset, 0.0), pos2(1.0 - texel_offset, 1.0));
            ui.add(Image::new(tex, GRADIENT_SIZE).tint(vertex_color).uv(uv))
                .on_hover_text(format!("A texture that is {} texels wide", g.0.len()));
            */
            ui.monospace(String::from("ASD DSA"));
            ui.monospace(String::from("lol"));
            egui::warn_if_debug_build(ui);
        });

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

// #![forbid(unsafe_code)]
// #![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
// #![warn(clippy::all, rust_2018_idioms)]
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
