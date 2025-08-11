use egui_inspect::{EguiInspect, EguiInspector};
use eframe::egui;

#[derive(EguiInspect, Default)]
struct MyApp {
    #[inspect(read_only)]
    string: String,
    #[inspect(multiline)]
    code: String,
    #[inspect(range(min = 12.0, max = 53.0))]
    unsigned32: u32,
    #[inspect(hidden)]
    #[allow(dead_code)]
    skipped: bool,
    #[inspect(tooltip = "A boolean")]
    boolean: bool,
    raw_string: &'static str,
    #[inspect(slider(min = "-43.0", max = 125.0))]
    float64: f32,
    #[inspect(name = "A proper field name")]
    ugly_internal_field_name: u16,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(EguiInspector::new(self));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native("EGui Inspector Simple Example", options, 
        Box::new(|_cc|
            Ok(Box::new(MyApp {
                raw_string:"A raw string which is not editable, even is read_only=false",
                string: "A read only string".to_string(),
                ..Default::default()
            }))
        )
    );
}