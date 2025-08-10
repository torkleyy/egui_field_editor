
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_inspect::{EguiInspect, EguiInspector};
use eframe::egui;

#[derive(EguiInspect, Debug, Default)]
pub struct TestData(
	#[inspect(name="Name tuple field")]
	String,
	#[inspect(name="float", read_only=true)]
	f32,
	#[inspect(name="float2")]
	f32,
	#[inspect(hidden)]
	MyEnum
);
#[derive(EguiInspect, Debug, Default, PartialEq)]
pub enum MyEnum {
	#[default]
	UnityVariant,
	TupleVariant1(
		#[inspect(slider, range(min=1.,max=12.))]
		u8
	),
	TupleVariant2(u8, u16),
	NamedVariant{a:f32, b:i64}
}
#[derive(EguiInspect, Debug, Default)]
pub struct Test {
	#[inspect(multiline=8)]
	pub multiline:String,
	pub vector: Vec<TestData>,
	pub u8: u8,
	#[inspect(range(min = 0., max = 12.0))]
	pub double: f64,
	#[inspect(slider, range(min = 0., max = 12.0))]
	pub float: f32,
	myenum:MyEnum
}


#[derive(EguiInspect, Default)]
struct MyApp(Test);

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let code = include_str!("simple.rs");
		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.add(EguiInspector::new(&mut self.0, false));
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