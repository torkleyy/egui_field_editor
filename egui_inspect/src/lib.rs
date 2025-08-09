//! # egui_inspect
//! This crate expose macros and traits to generate boilerplate code
//! for structs inspection and edition.
//!
//! Basic usage would be
//! ```
//! # use egui_inspect::*;
//! #[derive(EguiInspect)]
//! struct MyApp {
//!     #[inspect(read_only)]
//!     string: String,
//!     #[inspect(multiline)]
//!     code: String,
//!     #[inspect(range(min = 12.0, max = 53.0))]
//!     unsigned32: u32,
//!     #[inspect(hidden)]
//!     skipped: bool,
//!     #[inspect(tooltip = "A boolean")]
//!     boolean: bool,
//!     raw_string: &'static str,
//!     #[inspect(slider, range(min = "-43.0", max = 125.0))]
//!     float64: f32,
//!     #[inspect(name = "A proper field name")]
//!     ugly_internal_field_name: u16,
//! }
//!
//!
//! fn main() {
//!     let app = MyApp::default();
//!     app.inspect("My App", &ui); // here `ui` would be some `&mut egui::Ui`
//! }
//! ```
//!
//! You can add attributes to structures field.
//! Currently supported attributes are defined in the struct AttributeArgs of egui_inspect_derive
//!
//! Here is a list of supported attributes:
//!
//! - `name` *(String)*: Use custom label for the given field instead of the internal field name
//! - `hidden` *(bool)*: If true, doesn't generate code for the given field
//! - `read_only` *(bool)*: If true, the field is not editable (and color is grayed)
//! - `slider` *(bool)*: If true, use a slider when inspecting numbers (range must be present)
//! - `range` *(min=f32, max=f32)*: Min/Max value for inspecting numbers
//! - `multiline` *(bool)*: If true, display the text on multiple lines
//! - `tooltip` *(String)*: Tooltip to display when cursor is hover
//!


use egui::{Color32, Response, Ui, Widget};
#[cfg(feature = "nalgebra_glm")]
use nalgebra_glm::*;

/// See also [EguiInspect]
pub use egui_inspect_derive::*;

pub struct EguiInspector<'a, T : EguiInspect> {
	obj: &'a mut T
}
impl<'a, T : EguiInspect> EguiInspector<'a, T> {
	pub fn new(obj: &'a mut T) -> Self {
		Self { obj }
	}
	
}

impl<'a, T : EguiInspect> Widget for EguiInspector<'a, T> {
	fn ui(self, ui: &mut Ui) -> Response {
		ui.set_min_width(100.);
		let available_width = ui.available_width();

		ui.heading("Inspector");
		egui::ScrollArea::vertical().show(ui, |ui| {
			ui.set_min_width(available_width);
			self.obj.inspect("", "", false, ui);
		});

		ui.response()
	}

}

#[cfg(feature = "nalgebra_glm")]
macro_rules! impl_only_numbers_struct_inspect {
	($method:ident, $Type:ident, [$($field:ident),+]) => {
		//Useless: only expanded if feature is on
		//#[cfg(feature = "nalgebra_glm")]
		fn $method(data: &mut $Type, label: &str, ui: &mut egui::Ui) {
			ui.group(|ui| {
				ui.label(label);
				ui.horizontal(|ui| {
					$(
						ui.label(stringify!($field));
						ui.add(egui::DragValue::new(&mut data.$field).speed(0.1));
					)+
				});
			});
		}
	}
}
#[cfg(feature = "nalgebra_glm")]
#[derive(Clone, Debug, Copy)]
pub struct MyColor32(egui::Color32);
#[cfg(feature = "nalgebra_glm")]
impl std::ops::Deref for MyColor32 {
	type Target = egui::Color32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl From<Color32> for MyColor32 {
	fn from(value: Color32) -> Self {
		Self(value)
	}
}
impl From<MyColor32> for Color32 {
	fn from(value: MyColor32) -> Self {
		value.0
	}
}
#[cfg(feature = "nalgebra_glm")]
impl std::ops::DerefMut for MyColor32 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

/// Base trait to automatically inspect structs
pub trait EguiInspect {
	fn inspect(&mut self, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		self.inspect_with_custom_id(egui::Id::NULL, label, tooltip, read_only, ui);
	}
	fn inspect_with_custom_id(&mut self, parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui);

	fn add_widget<T: egui::Widget>(label: &str, widget: T, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		let available_width = ui.available_width();
		let label_width = available_width * 0.2;
		let field_width = 100.0f32.max(available_width * 0.8 - 10.0);
		ui.horizontal(|ui| {
			ui.add_enabled_ui(!read_only, |ui| {
				let r=ui.add_sized([label_width,0.],egui::Label::new(label).truncate().show_tooltip_when_elided(true).halign(egui::Align::LEFT));
				if !tooltip.is_empty() {
					if !read_only {
						r.on_hover_text(tooltip);
					} else {
						r.on_disabled_hover_text(tooltip);
					}
				}
				ui.spacing_mut().slider_width = field_width-50.; 
				ui.add_sized([field_width, 0.], widget);
			});
		}).response
	}
	fn add_custom_field<F>(
		label: &str,
		tooltip: &str,
		read_only: bool,
		ui: &mut egui::Ui,
		field_renderer: F,
	) -> egui::Response
	where
		F: FnOnce(&mut egui::Ui, f32),
	{
		let available_width = ui.available_width();
		let label_width = available_width * 0.2;
		let field_width = 100.0f32.max(available_width * 0.8 - 10.0);

		ui.horizontal(|ui| {
			ui.add_enabled_ui(!read_only, |ui| {
				let r = ui.add_sized(
					[label_width, 0.0],
					egui::Label::new(label)
						.truncate()
						.show_tooltip_when_elided(true)
						.halign(egui::Align::LEFT),
				);

				if !tooltip.is_empty() {
					if !read_only {
						r.on_hover_text(tooltip);
					} else {
						r.on_disabled_hover_text(tooltip);
					}
				}

				field_renderer(ui, field_width);
			});
		}).response
	}
	
	fn add_number_slider<Num: egui::emath::Numeric>(data: &mut Num, label: &str, tooltip: &str, read_only: bool, min:Num, max: Num, ui: &mut egui::Ui) {
		let editor=egui::Slider::new(data, min..=max);
		Self::add_custom_field(label, tooltip, read_only, ui, |ui, field_width| {
			ui.spacing_mut().slider_width = field_width-50.; 
			ui.add_sized([field_width, 0.], editor);
		});
	}
	fn add_number<Num: egui::emath::Numeric>(data: &mut Num, label: &str, tooltip: &str, read_only: bool, minmax: Option<(Num, Num)>, ui: &mut egui::Ui) {
		let mut editor=egui::DragValue::new(data);
		if let Some(minmax) = minmax {
			editor = editor.range(minmax.0..=minmax.1);
		}
		Self::add_widget(label, editor, tooltip, read_only, ui);
	}

	fn add_string_singleline<'t>(data: &'t mut dyn egui::TextBuffer, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		Self::add_widget(label, egui::TextEdit::singleline(data), tooltip, read_only, ui)
	}
	fn add_string_multiline<'t>(data: &'t mut dyn egui::TextBuffer, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		Self::add_widget(label, egui::TextEdit::multiline(data), tooltip, read_only, ui)
	}
	
	fn add_bool(data: &mut bool, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		Self::add_widget(label, egui::Checkbox::new(data, ""), tooltip, read_only, ui)
	}
	fn add_color32(data: &mut egui::Color32, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		let available_width = ui.available_width();
		let label_width = available_width * 0.2;
		//let field_width = 100.0f32.max(available_width * 0.8 - 10.0);
		ui.horizontal(|ui| {
			ui.add_enabled_ui(!read_only, |ui| {
				let r=ui.add_sized([label_width,0.],egui::Label::new(label).truncate().show_tooltip_when_elided(true).halign(egui::Align::LEFT));
				if !tooltip.is_empty() {
					if !read_only {
						r.on_hover_text(tooltip);
					} else {
						r.on_disabled_hover_text(tooltip);
					}
				}
			});
			ui.color_edit_button_srgba(data);
		}).response
	}
	fn add_color<T>(data: &mut T, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response where
		MyColor32: From<T>,
		T : From<MyColor32>,
		T : Clone {
		
		Self::add_custom_field(label, tooltip, read_only, ui, |ui, _field_width| {
			let mut color: MyColor32 = data.clone().into();
			if ui.color_edit_button_srgba(&mut color).changed() {
				*data = color.into();
			}
		})
			
	}
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2, Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3, Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4, Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_dvec2, DVec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_dvec3, DVec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_dvec4, DVec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2u8, U8Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3u8, U8Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4u8, U8Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2i8, I8Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3i8, I8Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4i8, I8Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2u16, U16Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3u16, U16Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4u16, U16Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2i16, I16Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3i16, I16Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4i16, I16Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2u32, U32Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3u32, U32Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4u32, U32Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2i32, I32Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3i32, I32Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4i32, I32Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2u64, U64Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3u64, U64Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4u64, U64Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec2i64, I64Vec2, [x, y]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec3i64, I64Vec3, [x, y, z]);
	#[cfg(feature = "nalgebra_glm")]
	impl_only_numbers_struct_inspect!(add_vec4i64, I64Vec4, [x, y, z, w]);
	#[cfg(feature = "nalgebra_glm")]
	fn add_vec3_color(data: &mut Vec3, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		Self::add_custom_field(label, tooltip, read_only, ui, |ui, _field_width| {
			let color: MyColor32 = (*data).into();
			let mut array = color.to_normalized_gamma_f32()[0..3].try_into().unwrap();
			if ui.color_edit_button_rgb(&mut array).changed() {
				*data = array.into();
			}
		})
	}
	#[cfg(feature = "nalgebra_glm")]
	fn add_vec4_color(data: &mut Vec4, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		Self::add_custom_field(label, tooltip, read_only, ui, |ui, _field_width| {
			let mut color: MyColor32 = (*data).into();
			if ui.color_edit_button_srgba(&mut color).changed() {
				*data = color.into();
			}
		})
	}
	#[cfg(feature = "nalgebra_glm")]
	fn add_vec3u8_color(data: &mut U8Vec3, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		Self::add_custom_field(label, tooltip, read_only, ui, |ui, _field_width| {
			let color: MyColor32 = (*data).into();
			let mut array = color.to_array()[0..3].try_into().unwrap();
			if ui.color_edit_button_srgb(&mut array).changed() {
				*data = array.into();
			}
		})
	}
	#[cfg(feature = "nalgebra_glm")]
	fn add_vec4u8_color(data: &mut U8Vec4, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) -> egui::Response {
		Self::add_custom_field(label, tooltip, read_only, ui, |ui, _field_width| {
			let mut color: MyColor32 = (*data).into();
			if ui.color_edit_button_srgba(&mut color).changed() {
				*data = color.into();
			}
		})
	}

}

pub trait DefaultEguiInspect : EguiInspect {
	fn default_inspect(&mut self, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		self.default_inspect_with_custom_id(egui::Id::NULL, label, tooltip, read_only, ui);
	}
	fn default_inspect_with_custom_id(&mut self, parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui);
}

impl<T: DefaultEguiInspect> EguiInspect for T {
	fn inspect(&mut self, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		self.default_inspect(label, tooltip, read_only, ui);
	}
	fn inspect_with_custom_id(&mut self, parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		self.default_inspect_with_custom_id(parent_id,label, tooltip, read_only, ui);
	}
}

pub mod base_type_inspect;
