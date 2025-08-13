
#![no_main]

use std::net::Ipv4Addr;

use egui::Color32;
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use egui_inspect::{EguiInspect, EguiInspector};
use eframe::egui;
#[cfg(target_arch = "wasm32")]
use eframe::web_sys;
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsCast;

#[derive(EguiInspect, Debug, Default)]
pub struct TestData(
	#[inspect(name="Name", tooltip="You can name tuple field")]
	String,
	#[inspect(name="A Float read only", read_only=true)]
	f32,
	#[inspect(name="A Float slider", slider(min=12.,max=155.))]
	f32,
	#[inspect(color, tooltip="not named => \"Field 3\"")]
	Color32,
	#[inspect(name="A Boxed string")]
	Box<String>,
	#[inspect(hidden)]
	#[allow(dead_code)]
	MyEnum
);
#[derive(EguiInspect, Debug, Default, PartialEq)]
pub enum MyEnum {
	#[default]
	UnitVariant,
	TupleVariant1(
		#[inspect(slider(min=1.,max=12.))]
		u8
	),
	TupleVariant2(
		#[inspect(name="Renamed Variant tuple field")]
		u8,
		#[inspect(custom_fn="inspect_num")]
		i16
	),
	NamedVariant{a:f32, b:i64},
	#[inspect(hidden)]
	IgnoredVariant,
	#[inspect(name="MyRenamedVariant")]
	RenamedVariant,
	#[inspect(read_only)]
	ReadOnlyVariant(u8, u8),
	SomeFieldReadOnlyVariantNamed {
		a:u8,
		#[inspect(read_only)]
		b:u8,
		#[inspect(hidden)]
		c:u8,
	},
	SomeFieldReadOnlyVariantTuple(
		u8,
		#[inspect(read_only)]
		u8,
		#[inspect(hidden)]
		u8,
	)
}

#[derive(EguiInspect)]
#[inspect(execute_btn(fn_name="println_ipv4"), execute_btn(fn_name="set_double_field_to_pi"))]
struct MyApp {
	#[inspect(multiline=8)]
	pub multiline:String,
	pub vector: Vec<TestData>,
	pub array: [TestData;4],
	pub u8: u8,
	#[inspect(range(min = 0., max = 12.0))]
	pub double: f64,
	#[inspect(slider(min = "-1000.", max = 12.0))]
	pub float: f32,
	pub my_enum:MyEnum,
	pub char:char,
	#[inspect(from_string)]
	pub ipv4: Ipv4Addr
}
impl Default for MyApp {
	fn default() -> Self {
		Self { multiline: Default::default(), vector: Default::default(), array: Default::default(), u8: Default::default(), double: Default::default(), float: Default::default(), my_enum: Default::default(), char: Default::default(), ipv4: Ipv4Addr::UNSPECIFIED }
	}
}
impl MyApp {
	fn set_double_field_to_pi(&mut self) {
		self.double = 3.1415;
	}
	fn println_ipv4(&self) {
		println!("{}", self.ipv4)
	}
}
fn inspect_num(data: &mut i16, label: &str, tooltip:&str, read_only: bool, ui: &mut egui::Ui) {
	egui_inspect::add_number(data, label, tooltip, read_only, None, ui);
}
impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let code = include_str!("advanced.rs");
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
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
    let app = MyApp::default();
    let web_options = eframe::WebOptions::default();
    let runner = eframe::WebRunner::new();

    let canvas = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("the_canvas_id"))
        .and_then(|elem| elem.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .expect("Failed to get canvas element");

    wasm_bindgen_futures::spawn_local(async move {
        runner
            .start(canvas, web_options, Box::new(|_cc| Ok(Box::new(app))))
            .await;
    });

    Ok(())
}
