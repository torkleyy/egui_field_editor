use proc_macro2::TokenStream;
use quote::{quote_spanned, quote};
use syn::spanned::Spanned;
use syn::Type;
use syn::{Field};

use crate::AttributeArgs;

#[allow(dead_code)]
pub fn get_path_str(type_path: &Type) -> Option<String> {
	Some(quote::ToTokens::to_token_stream(&type_path).to_string())
	/*match type_path {
		Type::Path(type_path) => {
			let ident = type_path
				.path
				.get_ident();
			println!("Path type path {:?}", ident);
			ident.map(|name| name.to_string())
		}
		Type::Reference(type_ref) => {
			if let Some(_) = type_ref.mutability {
				get_path_str(&type_ref.elem).map(|e| {format!("&mut {e}")})
			} else {
				get_path_str(&type_ref.elem).map(|e| {format!("&{e}")})
			}
		},
		Type::Array(type_array) =>  {
			get_path_str(&type_array.elem).map(|e| {format!("[{e}]")})
		},
		//Type::BareFn(type_bare_fn)  => get_path_str(&type_bare_fn.elem),
		//Type::Group(type_group) => { get_path_str(&type_group.elem) },
		//Type::ImplTrait(type_impl_trait) => get_path_str(&type_impl_trait.elem),
		//Type::Infer(type_infer) => get_path_str(&type_infer.elem),
		//Type::Macro(type_macro) => get_path_str(&type_macro.elem),
		//Type::Never(type_never) => get_path_str(&type_never.elem),
		Type::Paren(type_paren) => {
			get_path_str(&type_paren.elem).map(|e| {format!("({e})")})
		},
		Type::Ptr(type_ptr) => {
			get_path_str(&type_ptr.elem).map(|e| {format!("*{e}")})
		},
		Type::Slice(type_slice) => {
			get_path_str(&type_slice.elem).map(|e| {format!("{e}[..]")})
		},
		//Type::TraitObject(type_trait_object) => get_path_str(&type_trait_object.elem),
		//Type::Tuple(type_tuple) => get_path_str(&type_tuple.elem),
		//Type::Verbatim(token_stream) => get_path_str(&token_stream.elem),
		_ => {
			println!("No path {:?}", type_path);
			Some("".to_string())
		},
	}*/
}

fn prettify_name(s: &String) -> String {
	s.split('_')
		.filter(|part| !part.is_empty())
		.map(|word| {
			let split_index = word
				.char_indices()
				.rev()
				.find(|&(_, c)| !c.is_ascii_digit())
				.map(|(i, _)| i + 1)
				.unwrap_or(0);

			let (letters, digits) = word.split_at(split_index);

			let mut chars = letters.chars();
			let capitalized = match chars.next() {
				Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
				None => String::new(),
			};

			if digits.is_empty() {
				capitalized
			} else {
				format!("{} {}", capitalized, digits)
			}
		})
		.collect::<Vec<_>>()
		.join(" ")
}

pub(crate) fn get_function_call(field_access :TokenStream, field: &Field, attrs: &AttributeArgs, default_field_name:String) -> TokenStream {
	let name = &field.ident;

	let mut name_str = match &attrs.name {
		Some(n) => n.clone(),
		None => {
			if let Some(name) = name {
				name.to_string()
			} else {
				default_field_name
			}
		},
	};
	name_str = prettify_name(&name_str);
	let mut ro_activate = quote!{};
	if attrs.read_only {
		ro_activate = quote!{ui.disable();};
	}
	let range= &attrs.range;
	let mut tooltip_display = quote! {};
	if let Some(tooltip) = attrs.tooltip.as_ref() {
		tooltip_display = quote! {.response.on_hover_text(#tooltip)};
	}
	if attrs.slider {
		if let Some(range) = range {
			let min = range.min;
			let max = range.max;
			return quote_spanned! {field.span() => {
					ui.scope(|ui| {
						#ro_activate
						egui_inspect::InspectNumber::inspect_with_slider(#field_access, _parent_id, &#name_str, ui, #min, #max);
					})#tooltip_display;
				}
			};
		} else {
			return quote_spanned! { field.span() => {compile_error!("range is mandatory with slider".into()); } };
		}
	} else if let Some(range) = range {
		let min = range.min;
		let max = range.max;
		return quote_spanned! {field.span() => {
				ui.scope(|ui| {
					#ro_activate
					egui_inspect::InspectNumber::inspect_with_drag_value(#field_access, _parent_id, &#name_str, ui, Some((#min, #max)));
				})#tooltip_display;
			}
		};
	} else if attrs.multiline {
		return quote_spanned! {
			field.span() => {
				ui.scope(|ui| {
					#ro_activate
					egui_inspect::InspectString::inspect_multiline(#field_access, _parent_id, &#name_str, ui);
				})#tooltip_display;
			}
		};
	} else if attrs.color {
		return quote_spanned! {field.span() => {
				ui.scope(|ui| {
					#ro_activate
					egui_inspect::InspectColor::inspect_color(#field_access, _parent_id, &#name_str, ui);
				})#tooltip_display;
			}
		};
	}

	quote_spanned! {
		field.span() => {
			let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
			let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
			ui.scope(|ui| {
				#ro_activate
				egui_inspect::EguiInspect::inspect_with_custom_id(#field_access, parent_id, &#name_str, ui);
			})#tooltip_display;
		}
	}

}