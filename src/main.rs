#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // let ctx = egui::Context::default();

    // These no do trick here. see app itself
    // ctx.set_pixels_per_point(1.0); // Adjust this value to your preference
    // ctx.set_zoom_factor(5.0);

    // use egui::FontFamily::Proportional;
    // use egui::FontId;
    // use egui::TextStyle::*;

    // // Get current context style
    // let mut style = (*ctx.style()).clone();

    // // Redefine text_styles
    // style.text_styles = [
    //     (Heading, FontId::new(60.0, Proportional)),
    //     (Name("Heading2".into()), FontId::new(50.0, Proportional)),
    //     (Name("Context".into()), FontId::new(46.0, Proportional)),
    //     (Body, FontId::new(36.0, Proportional)),
    //     (Monospace, FontId::new(28.0, Proportional)),
    //     (Button, FontId::new(28.0, Proportional)),
    //     (Small, FontId::new(10.0, Proportional)),
    // ]
    // .into();

    // // Mutate global style with above changes
    // ctx.set_style(style);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "⌨️ Code Editor",
        native_options,
        Box::new(|cc| Box::new(eframe_template::CodeEditor::new(cc))),
    )
}
