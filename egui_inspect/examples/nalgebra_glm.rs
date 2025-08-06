#![cfg(feature = "nalgebra_glm")]

use egui::Color32;
use nalgebra_glm::*;


use egui_inspect::{DefaultEguiInspect, EGuiInspector};

use eframe::egui;


#[derive(DefaultEguiInspect, Debug, Default)]
pub struct TestColor {
	pub color:Color32,
	#[inspect(name="mycolor", color=true)]
	pub color2:Vec3,
	#[inspect(color=true)]
	pub color3:Vec4,
	#[inspect(name="data_test")]
	pub data: Vec<TestColor>,
	myenum:MyEnum
}
#[derive(DefaultEguiInspect, Debug, Default, PartialEq)]
pub enum MyEnum {
	#[default]
	Item1,
	Item2(
		#[inspect(color)]
		Vec3
	),
	Item3(
		#[inspect(color)]
		U8Vec3,
		#[inspect(color)]
		U8Vec4),
	Item4{vec2:Vec2, vec3:Vec3, i8vec2:I8Vec2}
}
#[derive(DefaultEguiInspect, Debug, Default)]
pub struct Test {
	pub data: Vec<TestColor>,
	pub pos: Vec3,
	pub pos2: U8Vec3,
}



#[derive(Default, DefaultEguiInspect)]
struct MyApp {
	test:Test
}



impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.add(EGuiInspector::new(self));
		});

	}
}

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Complete Example", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))));
}