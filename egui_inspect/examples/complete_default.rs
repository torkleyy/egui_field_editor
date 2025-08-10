
use egui_inspect::{EguiInspect, EguiInspector};
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui::Color32;
use eframe::egui;

macro_rules! generate_struct_tuple {
	($name:ident, [$($ty:ty),*]) => {
		#[derive(EguiInspect, Debug, Default)]
		pub struct $name(
			$(
				$ty,
				#[inspect(range(min = "-20.", max = 124.0), tooltip="A tooltip")]
				$ty,
				#[inspect(slider, range(min = "-10.", max = 12.0))]
				$ty,
			)*
		);
	};
}
generate_struct_tuple!(StructNumericTuple, [u8, u16, u32, u64, usize, f32, f64]);

#[derive(Default, EguiInspect, Debug, PartialEq)]
enum TestEnum {
	#[default]
	None,
	Tuple1(u8),
	Tuple2(
		#[inspect(read_only)]
		u8,
		#[inspect(hidden)]
		Enum2,
		f32
	),
	Tuple3(
		u8,
		#[inspect(range(min = 12., max=1500.))]
		u64,
		Color32,
		Enum2
	),
	Named{a_u8:u8, an_enum: Enum2},
	#[inspect(hidden)]
	#[allow(dead_code)]
	Hidden,
}
#[derive(Default, EguiInspect, Debug, PartialEq)]
enum Enum2 {
	#[default]
	None,
	Item1,
	Item2,
	Item3(Vec<f32>),
	#[inspect(hidden)]
	#[allow(dead_code)]
	Hidden,
}
#[derive(Default, EguiInspect, Debug, PartialEq)]
struct StructNamed {
	pub u8:u8,
	#[inspect(range(min = 0., max = 12.0))]
	pub u8_range:u8,
	#[inspect(slider, range(min = 1.0, max = 124.0))]
	pub u8_slider:u8,
	pub i16:i16,
	#[inspect(range(min = 1.0, max = 124.0))]
	pub i16_range:i16,
	#[inspect(slider, range(min = 1.0, max = 124.0))]
	pub i16_slider:i16,
	pub my_enum:TestEnum,
	pub color : Color32
}
#[derive(Default, EguiInspect)]
struct MyApp {
	#[inspect(read_only)]
	string: String,
	#[inspect(multiline)]
	code: String,
	#[inspect(range(min = 12.0, max = 53.0))]
	unsigned32: u32,
	#[allow(dead_code)]
	#[inspect(hidden)]
	skipped: bool,
	#[inspect(tooltip = "A boolean", name="My boolean")]
	boolean: bool,
	raw_string: &'static str,
	#[inspect(slider, range(min = "-43.0", max = 125.0))]
	float64: f32,
	#[inspect(name = "A proper field name")]
	ugly_internal_field_name: u16,
	#[inspect(name="Tuple Structure", tooltip="tooltip on structs are ignored")]
	struct_tuple: StructNumericTuple,
	#[inspect(name="Named Structure")]
	struct_named: StructNamed,
	#[inspect(name="Enum")]
	enumeration:TestEnum,
	option:Option<StructNamed>,
	vector:Vec<StructNamed>,
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let code = include_str!("complete_default.rs");
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
	let _ = eframe::run_native("EGui Inspector Complete Example", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))));
}
