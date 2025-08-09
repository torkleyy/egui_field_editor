use std::ops::Add;
use egui::{Color32, Ui};

macro_rules! impl_inspect_number {
	($($t:ty),+) => {
		$(
			impl crate::EguiInspect for $t {
				fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, enabled: bool, ui: &mut egui::Ui) {
					Self::add_number(self, label.into(), tooltip, enabled, None, ui);
				}
			}
		)*
	}
}

impl_inspect_number!(f32, f64);
impl_inspect_number!(i8, u8);
impl_inspect_number!(i16, u16);
impl_inspect_number!(i32, u32);
impl_inspect_number!(i64, u64);
impl_inspect_number!(isize, usize);


macro_rules! impl_inspect_mut_number {
	($($t:ty),+) => {
		$(
			impl crate::EguiInspect for &mut $t {
				fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, enabled: bool, ui: &mut egui::Ui) {
					Self::add_number(*self, label.into(), tooltip, enabled, None, ui);
				}
			}
		)*
	}
}
impl_inspect_mut_number!(f32, f64);
impl_inspect_mut_number!(i8, u8);
impl_inspect_mut_number!(i16, u16);
impl_inspect_mut_number!(i32, u32);
impl_inspect_mut_number!(i64, u64);
impl_inspect_mut_number!(isize, usize);

impl crate::EguiInspect for &'static str {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, _enabled: bool, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(label.to_owned() + ":").on_hover_text(tooltip).on_disabled_hover_text(tooltip);
			ui.colored_label(Color32::from_rgb(255, 0, 0), self.to_string())
				.on_hover_text("inspect_mut is not implemented for &'static str");
		});
	}
}

impl crate::EguiInspect for String {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, enabled: bool, ui: &mut egui::Ui) {
		Self::add_string_singleline(self, label, tooltip, enabled, ui);
	}
}

impl crate::EguiInspect for bool {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, enabled: bool, ui: &mut egui::Ui) {
		//ui.add_enabled(true, Checkbox::new(self, label));
		Self::add_bool(self, label, tooltip, enabled, ui);
	}
}

impl<T: crate::EguiInspect, const N: usize> crate::EguiInspect for [T; N] {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, enabled: bool, ui: &mut Ui) {
		let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
		let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
		egui::CollapsingHeader::new(label.to_string().add(format!("[{N}]").as_str())).show(ui, |ui| {
			for (i, item) in self.iter_mut().enumerate() {
				item.inspect_with_custom_id(parent_id, format!("Item {i}").as_str(), tooltip, enabled, ui);
			}
		});
	}
}

impl<T: crate::EguiInspect + Default> crate::EguiInspect for Vec<T> {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, enabled: bool, ui: &mut Ui) {
		let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
		let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
		ui.horizontal_top(|ui| {
			egui::CollapsingHeader::new(label.to_string().add(format!("[{}]", self.len()).as_str())).id_salt(id)
				.show(ui, |ui| {
				for (i, item) in self.iter_mut().enumerate() {
					item.inspect_with_custom_id(parent_id, format!("Item {i}").as_str(), tooltip, enabled, ui);
				}
			});

			let response = ui.button("+");
			if response.clicked() {
				self.push(T::default());
			}

			let response = ui.button("-");
			if response.clicked() {
				self.pop();
			}
		});
	}
}

impl crate::EguiInspect for Color32 {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, enabled: bool, ui: &mut egui::Ui) {
		Self::add_color(self, label, tooltip, enabled, ui);
	}
}




#[cfg(feature = "nalgebra_glm")]
mod nalgebra_ui {
	use egui::Color32;
	use nalgebra_glm::*;
	use crate::EguiInspect;
	use crate::MyColor32;

	macro_rules! impl_only_numbers_struct_inspect {
	($Type:ident, [$($field:ident),+]) => {
		impl EguiInspect for $Type {
			fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, enabled: bool, ui: &mut egui::Ui) {
				Self::add_custom_field(label, tooltip, enabled, ui, |ui, _field_size| {
					ui.horizontal(|ui| {
						$(
							ui.label(stringify!($field));
							ui.add(egui::DragValue::new(&mut self.$field).speed(0.1));
						)+
					});
				});
			}
		}
	};
}

	impl_only_numbers_struct_inspect!(Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(DVec2, [x, y]);
	impl_only_numbers_struct_inspect!(DVec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(DVec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(U8Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(U8Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(U8Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(I8Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(I8Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(I8Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(U16Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(U16Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(U16Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(I16Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(I16Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(I16Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(U32Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(U32Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(U32Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(I32Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(I32Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(I32Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(U64Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(U64Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(U64Vec4, [x, y, z, w]);
	impl_only_numbers_struct_inspect!(I64Vec2, [x, y]);
	impl_only_numbers_struct_inspect!(I64Vec3, [x, y, z]);
	impl_only_numbers_struct_inspect!(I64Vec4, [x, y, z, w]);


	impl From<MyColor32> for Vec3 {
		fn from(value: MyColor32) -> Self {
			Vec3::new(value.0.r() as f32 / 255.,value.0.g() as f32 / 255.,value.0.b() as f32 / 255.)
		}
	}
	impl From<Vec3> for MyColor32 {
		fn from(value: Vec3) -> Self {
			Self(Color32::from_rgb((value.x * 255.) as u8,(value.y * 255.) as u8,(value.z * 255.) as u8))
		}
	}

	impl From<MyColor32> for Vec4 {
		fn from(value: MyColor32) -> Self {
			Vec4::new(value.0.r() as f32 / 255.,value.0.g() as f32 / 255.,value.0.b() as f32 / 255.,value.0.a() as f32 / 255.)
		}
	}
	impl From<Vec4> for MyColor32 {
		fn from(value: Vec4) -> Self {
			Self(Color32::from_rgba_premultiplied((value.x * 255.) as u8,(value.y * 255.) as u8,(value.z * 255.) as u8,(value.w * 255.) as u8))
		}
	}

	impl From<MyColor32> for U8Vec3 {
		fn from(value: MyColor32) -> Self {
			U8Vec3::new(value.0.r(),value.0.g(),value.0.b())
		}
	}
	impl From<U8Vec3> for MyColor32 {
		fn from(value: U8Vec3) -> Self {
			Self(Color32::from_rgb(value.x,value.y,value.z))
		}
	}

	impl From<MyColor32> for U8Vec4 {
		fn from(value: MyColor32) -> Self {
			U8Vec4::new(value.0.r(),value.0.g(),value.0.b(),value.0.a())
		}
	}
	impl From<U8Vec4> for MyColor32 {
		fn from(value: U8Vec4) -> Self {
			Self(Color32::from_rgba_premultiplied(value.x,value.y,value.z,value.w))
		}
	}

}
