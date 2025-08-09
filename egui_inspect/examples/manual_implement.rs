use egui::{Color32, TextEdit};
use egui_inspect::EguiInspector;

impl Default for MyStruct {
	fn default() -> Self {
		Self { a_bool: Default::default(), an_int: Default::default(), an_uint: Default::default(), a_float: Default::default(), a_color: Color32::from_rgb(127, 0, 200), a_string: String::from("A single line string"), a_second_string: String::from("A\nmultiline\nline\nstring") }
	}
}
impl eframe::App for MyStruct {
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

struct MyStruct {
	a_bool:bool,
	an_int:i32,
	an_uint:u64,
	a_float:f32,
	a_color:egui::Color32,
	a_string:String,
	a_second_string:String,
}
impl egui_inspect::EguiInspect for MyStruct {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, _tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
		let _parent_id_to_provide_to_children = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
		let mut add_content=|ui:&mut egui::Ui| {
			egui_inspect::add_bool(&mut self.a_bool, "Bool", "Boolean Tooltip", read_only, ui);
			egui_inspect::add_number(&mut self.an_int, "Integer", "Integer Tooltip", read_only, None, ui);
			egui_inspect::add_number(&mut self.an_uint, "Unsigned Integer", "Unsigned Integer Tooltip with min/max", read_only, Some((12, 50000)), ui);
			egui_inspect::add_number_slider(&mut self.a_float, "Float", "Float Slider Tooltip", read_only, -12., 50., ui);
			egui_inspect::add_color(&mut self.a_color, "Color", "", read_only, ui);
			egui_inspect::add_string_singleline(&mut self.a_string, "String", "", read_only, ui);
			egui_inspect::add_string_multiline(&mut self.a_second_string, "Multiline String", "", read_only, 4, ui);
		};
		if !label.is_empty() {
			egui::CollapsingHeader::new(label).id_salt(id).show(ui, add_content);
		} else {
			add_content(ui);
		}
	}
}

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Simple Example", options, Box::new(|_cc| Ok(Box::new(MyStruct::default()))));
}