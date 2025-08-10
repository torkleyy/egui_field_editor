use std::ops::Add;
use egui::{Color32, Ui};
use crate::EguiInspect;

macro_rules! impl_inspect_number {
	($($t:ty),+) => {
		$(
			impl crate::EguiInspect for $t {
				fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
					crate::add_number(self, label.into(), tooltip, read_only, None, ui);
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

impl<T:EguiInspect> EguiInspect for &mut T {
	fn inspect_with_custom_id(&mut self, parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		<T as EguiInspect>::inspect_with_custom_id(*self, parent_id, label, tooltip, read_only, ui);
	}
}

impl crate::EguiInspect for &'static str {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, _read_only: bool, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(label.to_owned() + ":").on_hover_text(tooltip).on_disabled_hover_text(tooltip);
			ui.colored_label(Color32::from_rgb(255, 0, 0), self.to_string())
				.on_hover_text("inspect_mut is not implemented for &'static str");
		});
	}
}

impl crate::EguiInspect for String {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		crate::add_string_singleline(self, label, tooltip, read_only, ui);
	}
}

impl crate::EguiInspect for bool {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		crate::add_bool(self, label, tooltip, read_only, ui);
	}
}

impl<T: crate::EguiInspect, const N: usize> crate::EguiInspect for [T; N] {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut Ui) {
		let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
		let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
		egui::CollapsingHeader::new(label.to_string().add(format!("[{N}]").as_str())).show(ui, |ui| {
			for (i, item) in self.iter_mut().enumerate() {
				item.inspect_with_custom_id(parent_id, format!("Item {i}").as_str(), tooltip, read_only, ui);
			}
		});
	}
}

struct EnumeratedItem<T> {
	item: T,
	index: usize,
}

impl<T: crate::EguiInspect + Default> egui_dnd::DragDropItem for EnumeratedItem<&mut T> {
	fn id(&self) -> egui::Id {
		egui::Id::new(self.index)
	}
}

impl<T: crate::EguiInspect + Default> crate::EguiInspect for Vec<T> {
	fn inspect_with_custom_id(
		&mut self,
		parent_id: egui::Id,
		label: &str,
		tooltip: &str,
		read_only: bool,
		ui: &mut Ui,
	) {
		let id = if parent_id == egui::Id::NULL {
			ui.next_auto_id()
		} else {
			parent_id.with(label)
		};

		ui.horizontal_top(|ui| {
			egui::CollapsingHeader::new(format!("{label} [{}]", self.len()))
				.id_salt(id)
				.show(ui, |ui| {
					let response = egui_dnd::dnd(ui, id.with("dnd"))
					.with_animation_time(0.0)
					.show(
						self
							.iter_mut()
							.enumerate()
							.map(|(i, item)| EnumeratedItem { item, index: i }),
						|ui, item, handle, state| {
							ui.horizontal(|ui| {
								handle.ui(ui, |ui| {
									if state.dragged {
										ui.label("≡");
									} else {
										ui.label("☰");
									}
								});
								let index = item.index;
								item.item.inspect_with_custom_id(parent_id, format!("Item {index}").as_str(), tooltip, read_only, ui);
							});
						},
					);
				if response.is_drag_finished() {
					response.update_vec(self);
				}
			});
		});

		ui.add_enabled_ui(!read_only, |ui| {
			ui.horizontal_top(|ui| {
				ui.add_space(ui.available_width() - 50.);
				if ui.button("+").clicked() {
					self.push(T::default());
				}
				if ui.button("-").clicked() {
					self.pop();
				}
			});
		});
	}

}

impl crate::EguiInspect for Color32 {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		crate::add_color(self, label, tooltip, read_only, ui);
	}
}

impl<T : EguiInspect> crate::EguiInspect for Option<T>
	where T : Default+PartialEq {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		let id = if _parent_id == egui::Id::NULL {
			ui.next_auto_id()
		} else {
			_parent_id.with(label)
		};
		let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
		let available_width = ui.available_width();
		let label_width = available_width * 0.2;
		let field_width = 100.0f32.max(available_width * 0.8 - 10.0);

		ui.horizontal(|ui| {
			let r = ui.add_sized(
				[label_width, 0.0],
				egui::Label::new(label)
					.truncate()
					.show_tooltip_when_elided(true)
					.halign(egui::Align::LEFT),
			);

			if !tooltip.is_empty() {
				r.on_hover_text(tooltip).on_disabled_hover_text(tooltip);
			}
			ui.add_enabled_ui(!read_only, |ui| {
				egui::ComboBox::from_id_salt(id)
					.selected_text(
						match self {
							None => "None",
							Some(_) => "Some"
						},
					)
					.width(field_width)
					.show_ui(
						ui,
						|ui| {
							if ui.selectable_value(self, None, "None").changed() {
								*self = None;
							}
							if ui
								.selectable_value(
									self,
									Some(Default::default()),
									"Some",
								)
								.changed()
							{
								*self = Some(Default::default());
							}
						},
					);
			});
		});
		match self {
			None => {}
			Some(field0) => {
				ui.indent(id, |ui| {
					field0.inspect_with_custom_id(
						parent_id,
						&"",
						"",
						read_only,
						ui,
					);
				});
			}
		}
	}
}



#[cfg(feature = "nalgebra_glm")]
mod nalgebra_ui {
	use egui::Color32;
	use nalgebra_glm::*;
	use crate::EguiInspect;
	use crate::Color32Wrapper;

	macro_rules! impl_only_numbers_struct_inspect {
		($Type:ident, [$($field:ident),+]) => {
			impl EguiInspect for $Type {
				fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
					crate::add_custom_field(label, tooltip, read_only, ui, |ui, _field_size| {
						ui.group(|ui| {
							ui.horizontal(|ui| {
							$(
								ui.label(stringify!($field));
								ui.add(egui::DragValue::new(&mut self.$field).speed(0.1));
							)+
							});
						});
					});
				}
			}
		};
	}
	macro_rules! impl_mat_inspect {
		($Type:ident, [$( [$($field:ident),+] ),+]) => {
			impl EguiInspect for $Type {
				fn inspect_with_custom_id(
					&mut self,
					_parent_id: egui::Id,
					label: &str,
					tooltip: &str,
					read_only: bool,
					ui: &mut egui::Ui,
				) {
					crate::add_custom_field(label, tooltip, read_only, ui, |ui, _field_size| {
						ui.vertical(|ui| {
							ui.group(|ui| {
								$(
									ui.horizontal(|ui| {
										$(
											ui.label(stringify!($field));
											ui.add(egui::DragValue::new(&mut self.$field).speed(0.1));
										)+
									});
								)+
							});
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
	impl_only_numbers_struct_inspect!(Quat, [i, j, k, w]);
	impl_only_numbers_struct_inspect!(DQuat, [i, j, k, w]);
	impl_mat_inspect!(Mat2x2, [[m11, m12], [m21, m22]]);
	impl_mat_inspect!(Mat2x3, [[m11, m12, m13], [m21, m22, m23]]);
	impl_mat_inspect!(Mat2x4, [[m11, m12, m13, m14], [m21, m22, m23, m24]]);
	impl_mat_inspect!(Mat3x2, [[m11, m12], [m21, m22], [m31, m32]]);
	impl_mat_inspect!(Mat3x3, [[m11, m12, m13], [m21, m22, m23], [m31, m32, m33]]);
	impl_mat_inspect!(Mat3x4, [[m11, m12, m13, m14], [m21, m22, m23, m24], [m31, m32, m33, m34]]);
	impl_mat_inspect!(Mat4x2, [[m11, m12], [m21, m22], [m31, m32], [m41, m42]]);
	impl_mat_inspect!(Mat4x3, [[m11, m12, m13], [m21, m22, m23], [m31, m32, m33], [m41, m42, m43]]);
	impl_mat_inspect!(Mat4x4, [[m11, m12, m13, m14], [m21, m22, m23, m24], [m31, m32, m33, m34], [m41, m42, m43, m44]]);
	
	impl From<Color32Wrapper> for Vec3 {
		fn from(value: Color32Wrapper) -> Self {
			Vec3::new(value.0.r() as f32 / 255.,value.0.g() as f32 / 255.,value.0.b() as f32 / 255.)
		}
	}
	impl From<Vec3> for Color32Wrapper {
		fn from(value: Vec3) -> Self {
			Self(Color32::from_rgb((value.x * 255.) as u8,(value.y * 255.) as u8,(value.z * 255.) as u8))
		}
	}

	impl From<Color32Wrapper> for Vec4 {
		fn from(value: Color32Wrapper) -> Self {
			Vec4::new(value.0.r() as f32 / 255.,value.0.g() as f32 / 255.,value.0.b() as f32 / 255.,value.0.a() as f32 / 255.)
		}
	}
	impl From<Vec4> for Color32Wrapper {
		fn from(value: Vec4) -> Self {
			Self(Color32::from_rgba_premultiplied((value.x * 255.) as u8,(value.y * 255.) as u8,(value.z * 255.) as u8,(value.w * 255.) as u8))
		}
	}

	impl From<Color32Wrapper> for U8Vec3 {
		fn from(value: Color32Wrapper) -> Self {
			U8Vec3::new(value.0.r(),value.0.g(),value.0.b())
		}
	}
	impl From<U8Vec3> for Color32Wrapper {
		fn from(value: U8Vec3) -> Self {
			Self(Color32::from_rgb(value.x,value.y,value.z))
		}
	}

	impl From<Color32Wrapper> for U8Vec4 {
		fn from(value: Color32Wrapper) -> Self {
			U8Vec4::new(value.0.r(),value.0.g(),value.0.b(),value.0.a())
		}
	}
	impl From<U8Vec4> for Color32Wrapper {
		fn from(value: U8Vec4) -> Self {
			Self(Color32::from_rgba_premultiplied(value.x,value.y,value.z,value.w))
		}
	}

}
#[cfg(feature = "datepicker")]
mod datepicker {
	use std::hash::{Hash, Hasher};

	use crate::EguiInspect;
	use chrono::prelude::*;
	use egui_extras::DatePickerButton;
	impl EguiInspect for NaiveDate {
		fn inspect_with_custom_id(&mut self, parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
			let id = if parent_id == egui::Id::NULL { egui::Id::NULL } else { parent_id.with(label) };
			let widget = DatePickerButton::new(self);
			if id != egui::Id::NULL {
				// Ugly hack because DatePickerButton::id_salt() taking a &str
				let mut hasher = std::hash::DefaultHasher::new();
				id.hash(&mut hasher);
				crate::add_widget(label, widget.id_salt(format!("{}", hasher.finish()).as_str()), tooltip, read_only, ui);
			} else {
				crate::add_widget(label, widget, tooltip, read_only, ui);
			}
		}
	}
}