

use egui::TextEdit;
use egui_inspect::{DefaultEguiInspect, EguiInspector};

use eframe::egui;



#[derive(DefaultEguiInspect, Debug, Default)]
pub struct TestData {
	#[inspect(name="plip", multiline=false)]
	pub ploup:String,
	#[inspect(name="data_test")]
	pub data: Vec<TestData>,
	#[inspect(read_only=false, hidden=false)]
	pub f32:f32,
	//#[inspect(multiline=true)]
	pub f32_2:f32,
	myenum:MyEnum
}
#[derive(DefaultEguiInspect, Debug, Default, PartialEq)]
pub enum MyEnum {
	#[default]
	Item1,
	Item2(
		#[inspect(slider, range(min=1.,max=12.))]
		u8
	),
	Item3(u8, u16),
	Item4{a:f32, b:i64}
}
#[derive(DefaultEguiInspect, Debug, Default)]
pub struct Test {
	#[inspect(multiline=true)]
	pub ploup:String,
	pub data: Vec<TestData>,
	pub u8: u8,

	#[inspect(range(min = 0., max = 12.0))]
	pub double: f64,
}

#[derive(DefaultEguiInspect, Debug, Default)]
pub struct Test2(u32, TestData);

#[derive(DefaultEguiInspect, Default)]
struct MyApp {
	test: Test,
	test2: Test2
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let mut code = include_str!("simple.rs");
		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.add(EguiInspector::new(self));
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::ScrollArea::vertical().id_salt("code_scrolling").show(ui, |ui| {
				ui.add_sized(ui.available_size(), TextEdit::multiline(&mut code).code_editor());
			});
		});
	}
}

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Simple Example", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))));
}