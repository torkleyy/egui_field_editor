use crate::utils::get_path_str;
use crate::AttributeArgs;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::Field;


pub(crate) fn path_is_internally_handled(path_str: &String) -> bool {
	path_str == "f32"
		|| path_str == "f64"
		|| path_str == "u8"
		|| path_str == "i8"
		|| path_str == "u16"
		|| path_str == "i16"
		|| path_str == "u32"
		|| path_str == "i32"
		|| path_str == "u64"
		|| path_str == "i64"
		|| path_str == "usize"
		|| path_str == "isize"
		|| path_str == "bool"
		|| path_str == "String"
		|| path_str == "str"
		|| path_str == "Vec3"
		|| path_str == "Vec4"
}

pub(crate) fn try_handle_internal_path(
	field_name :TokenStream,
	field: &Field,
	attrs: &AttributeArgs,
	default_field_name:String
) -> Option<TokenStream> {
	let path_str = get_path_str(&field.ty);

	let path_str = path_str.as_ref()?;

	if !path_is_internally_handled(path_str) {
		return None;
	}
	match path_str.as_str() {
		"f64" | "f32" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" => {
			handle_number_path(field_name, field, attrs, default_field_name)
		}
		"String" => handle_string_path(field_name, field, attrs, default_field_name),
		"Vec3" | "Vec4" | "U8Vec3" | "U8Vec4" => {
			handle_color_vec_path(field_name, field, attrs, default_field_name)
		}
		_ => None,
	}
}

fn handle_number_path(field_name :TokenStream, field: &Field, attrs: &AttributeArgs, default_field_name:String) -> Option<TokenStream> {
	print!("{} : {:?}",field_name, attrs);
	let name = &field.ident;

	let name_str = match &attrs.name {
		Some(n) => n.clone(),
		None => {
			if let Some(name) = name {
				name.clone().to_string()
			} else {
				default_field_name
			}
		},
	};

	let range= &attrs.range;
	
	let mut ro_activate = quote! {};
	let mut ro_desactivate = quote! {};
	if attrs.read_only {
		ro_activate = quote! {ui.disable();};
		ro_desactivate = quote! {};
	}

	if attrs.slider && range.is_some() {
		let range = range.as_ref().unwrap();
		let min = range.min;
		let max = range.max;
		return Some(quote_spanned! {field.span() => {
				#ro_activate
				egui_inspect::InspectNumber::inspect_with_slider(#field_name, _parent_id, &#name_str, ui, #min, #max);
				#ro_desactivate
			}
		})
	} else if let Some(range) = range {
		let min = range.min;
		let max = range.max;
		return Some(quote_spanned! {field.span() => {
				#ro_activate
				egui_inspect::InspectNumber::inspect_with_drag_value(#field_name, _parent_id, &#name_str, ui, Some((#min, #max)));
				#ro_desactivate
			}
		})
	} else {
		return Some(quote_spanned! {field.span() => {
				#ro_activate
				egui_inspect::InspectNumber::inspect_with_drag_value(#field_name, _parent_id, &#name_str, ui, None);
				#ro_desactivate
			}
		})
	}
}

fn handle_string_path(field_name :TokenStream, field: &Field, attrs: &AttributeArgs, default_field_name:String) -> Option<TokenStream> {
	let name = &field.ident;

	let name_str = match &attrs.name {
		Some(n) => n.clone(),
		None =>  {
			if let Some(name) = name {
				name.clone().to_string()
			} else {
				default_field_name
			}
		},
	};

	let multiline = attrs.multiline;
	let read_only = attrs.read_only;

	let mut ro_activate = quote! {};
	let mut ro_desactivate = quote! {};
	if read_only {
		ro_activate = quote! {ui.disable();};
		ro_desactivate = quote! {};
	}

	if multiline {
		Some(quote_spanned! {field.span() => {
			#ro_activate
			egui_inspect::InspectString::inspect_multiline(#field_name, _parent_id, &#name_str, ui);
			#ro_desactivate
			}
		})
	} else {
		Some(quote_spanned! {field.span() => {
			#ro_activate
			egui_inspect::InspectString::inspect_singleline(#field_name, _parent_id, &#name_str, ui);
			#ro_desactivate
			}
		})
	}
}

fn handle_color_vec_path(field_name :TokenStream, field: &Field, attrs: &AttributeArgs, default_field_name:String) -> Option<TokenStream> {
	let name = &field.ident;

	let name_str = match &attrs.name {
		Some(n) => n.clone(),
		None =>  {
			if let Some(name) = name {
				name.clone().to_string()
			} else {
				default_field_name
			}
		},
	};

	let color = attrs.color;
	let read_only = attrs.read_only;
	let mut ro_activate = quote! {};
	let mut ro_desactivate = quote! {};
	if read_only {
		ro_activate = quote! {ui.disable();};
		ro_desactivate = quote! {};
	}

	if color {
		Some(quote_spanned! {field.span() => {
				#ro_activate
				egui_inspect::InspectColor::inspect_color(#field_name, _parent_id, &#name_str, ui);
				#ro_desactivate
			}
		})
	} else {
		Some(quote_spanned! {field.span() => {
				#ro_activate
				egui_inspect::EguiInspect::inspect_with_custom_id(#field_name, _parent_id, &#name_str, ui);
				#ro_desactivate
			}
		})
	}
}