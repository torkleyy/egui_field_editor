#![cfg(feature = "datepicker")]
use chrono::NaiveDate;
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_inspect::{EguiInspect, EguiInspector};
use eframe::egui;

#[derive(EguiInspect, Debug, Default)]
pub struct Test {
	pub naive_date:NaiveDate,
	#[inspect(date(calendar_week=false, highlight_weekends=false))]
	pub naive_date2:NaiveDate,

}


#[derive(EguiInspect, Default)]
struct MyApp(Test);

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let code = include_str!("simple.rs");
		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.add(EguiInspector::new(self));
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().id_salt("code_scrolling").show(ui, |ui| {
				code_view_ui(ui, &CodeTheme::default(), code, "rust");
			});
		});
	}
}

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Simple Example", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))));
}