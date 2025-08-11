#![cfg(feature = "datepicker")]
use chrono::NaiveDate;
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_inspect::{EguiInspect, EguiInspector};
use eframe::egui;



#[derive(EguiInspect)]
struct MyApp{
	pub naive_date:NaiveDate,
	#[inspect(date(calendar_week=false, highlight_weekends=false, start_end_years(min=2015, max=2028), combo_boxes=false))]
	pub naive_date2:NaiveDate,
}
impl Default for MyApp {
	fn default() -> Self {
		Self { naive_date: chrono::Local::now().date_naive(), naive_date2: chrono::Local::now().date_naive() }
	}
}
impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let code = include_str!("datepicker.rs");
		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.add(EguiInspector::new(self).with_title("Inpector"));
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().id_salt("code_scrolling").show(ui, |ui| {
				code_view_ui(ui, &CodeTheme::default(), code, "Rust");
			});
		});
	}
}

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Datepicker Example", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))));
}