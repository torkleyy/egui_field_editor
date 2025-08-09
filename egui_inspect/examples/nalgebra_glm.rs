#![cfg(feature = "nalgebra_glm")]

use egui::TextEdit;
use nalgebra_glm::*;


use egui_inspect::{EguiInspect, EguiInspector};

use eframe::egui;

#[derive(EguiInspect, Default, PartialEq)]
pub enum TestEnum {
	#[default]
	None,
	VectorsTuple(
		Vec2,
		Vec3,
		Vec4,
		TVec2<f64>,
		TVec3<f64>,
		TVec4<f64>,
		U8Vec2,
		U8Vec3,
		U8Vec4,
		I8Vec2,
		I8Vec3,
		I8Vec4,
		U16Vec2,
		U16Vec3,
		U16Vec4,
		I16Vec2,
		I16Vec3,
		I16Vec4,
		U32Vec2,
		U32Vec3,
		U32Vec4,
		I32Vec2,
		I32Vec3,
		I32Vec4,
		U64Vec2,
		U64Vec3,
		U64Vec4,
		I64Vec2,
		I64Vec3,
		I64Vec4,
	),
	ColorsTuple(
		#[inspect(color)]
		Vec3,
		#[inspect(color)]
		Vec4,
		#[inspect(color)]
		U8Vec3,
		#[inspect(color)]
		U8Vec4,
	),
	VectorsList(
		Vec<Vec3>,
	),
	VectorsNamed {
		vec2: Vec2,
		vec3: Vec3,
		vec4: Vec4,
		vec2f64: TVec2<f64>,
		vec3f64: TVec3<f64>,
		vec4f64: TVec4<f64>,
		vec2u8: U8Vec2,
		vec3u8: U8Vec3,
		vec4u8: U8Vec4,
		vec2i8: I8Vec2,
		vec3i8: I8Vec3,
		vec4i8: I8Vec4,
		vec2u16: U16Vec2,
		vec3u16: U16Vec3,
		vec4u16: U16Vec4,
		vec2i16: I16Vec2,
		vec3i16: I16Vec3,
		vec4i16: I16Vec4,
		vec2u32: U32Vec2,
		vec3u32: U32Vec3,
		vec4u32: U32Vec4,
		vec2i32: I32Vec2,
		vec3i32: I32Vec3,
		vec4i32: I32Vec4,
		vec2u64: U64Vec2,
		vec3u64: U64Vec3,
		vec4u64: U64Vec4,
		vec2i64: I64Vec2,
		vec3i64: I64Vec3,
		vec4i64: I64Vec4,
	},
	ColorsNamed{
		#[inspect(color)]
		vec3: Vec3,
		#[inspect(color)]
		vec4: Vec4,
		#[inspect(color)]
		vec3u8: U8Vec3,
		#[inspect(color)]
		vec4u8: U8Vec4,
	},
}
#[derive(EguiInspect, Default)]
pub struct TestNamedStructColors {
	#[inspect(color)]
	pub vec3: Vec3,
	#[inspect(color)]
	pub vec4: Vec4,
	#[inspect(color)]
	pub vec3u8: U8Vec3,
	#[inspect(color)]
	pub vec4u8: U8Vec4,
}

#[derive(EguiInspect, Default)]
pub struct TestNamedStructVectors {
	pub vec2: Vec2,
	pub vec3: Vec3,
	pub vec4: Vec4,
	pub vec2f64: TVec2<f64>,
	pub vec3f64: TVec3<f64>,
	pub vec4f64: TVec4<f64>,
	pub vec2u8: U8Vec2,
	pub vec3u8: U8Vec3,
	pub vec4u8: U8Vec4,
	pub vec2i8: I8Vec2,
	pub vec3i8: I8Vec3,
	pub vec4i8: I8Vec4,
	pub vec2u16: U16Vec2,
	pub vec3u16: U16Vec3,
	pub vec4u16: U16Vec4,
	pub vec2i16: I16Vec2,
	pub vec3i16: I16Vec3,
	pub vec4i16: I16Vec4,
	pub vec2u32: U32Vec2,
	pub vec3u32: U32Vec3,
	pub vec4u32: U32Vec4,
	pub vec2i32: I32Vec2,
	pub vec3i32: I32Vec3,
	pub vec4i32: I32Vec4,
	pub vec2u64: U64Vec2,
	pub vec3u64: U64Vec3,
	pub vec4u64: U64Vec4,
	pub vec2i64: I64Vec2,
	pub vec3i64: I64Vec3,
	pub vec4i64: I64Vec4,
}

#[derive(EguiInspect, Default)]
pub struct TestTupleStructColors(
	#[inspect(color)]
	pub Vec3,
	#[inspect(color)]
	pub Vec4,
	#[inspect(color)]
	pub U8Vec3,
	#[inspect(color)]
	pub U8Vec4,
);

#[derive(EguiInspect, Default)]
pub struct TestTupleStructVectors(
	pub Vec2,
	pub Vec3,
	pub Vec4,
	pub TVec2<f64>,
	pub TVec3<f64>,
	pub TVec4<f64>,
	pub U8Vec2,
	pub U8Vec3,
	pub U8Vec4,
	pub I8Vec2,
	pub I8Vec3,
	pub I8Vec4,
	pub U16Vec2,
	pub U16Vec3,
	pub U16Vec4,
	pub I16Vec2,
	pub I16Vec3,
	pub I16Vec4,
	pub U32Vec2,
	pub U32Vec3,
	pub U32Vec4,
	pub I32Vec2,
	pub I32Vec3,
	pub I32Vec4,
	pub U64Vec2,
	pub U64Vec3,
	pub U64Vec4,
	pub I64Vec2,
	pub I64Vec3,
	pub I64Vec4,
);



#[derive(Default, EguiInspect)]
struct MyApp {
	test_named_vectors:TestNamedStructVectors,
	test_named_colors:TestNamedStructColors,
	test_tuple_vectors:TestTupleStructVectors,
	test_tuple_colors:TestTupleStructColors,
	test_enum:TestEnum
}



impl eframe::App for MyApp {
	
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let mut code = include_str!("nalgebra_glm.rs");
		egui::SidePanel::right("right_panel").show(ctx, |ui| {
			ui.add(EguiInspector::new(self, false));
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
	let _ = eframe::run_native("EGui Inspector NAlgebra Example", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))));
}