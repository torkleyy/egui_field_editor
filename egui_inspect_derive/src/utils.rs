use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::Type;
use syn::{Field};

use crate::AttributeArgs;

#[allow(dead_code)]
pub fn get_path_str(type_path: &Type) -> Option<String> {
	Some(quote::ToTokens::to_token_stream(&type_path).to_string())
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

	let name_str = match &attrs.name {
		Some(n) => n.clone(),
		None => {
			if let Some(name) = name {
				prettify_name(&name.to_string())
			} else {
				prettify_name(&default_field_name.to_string())
			}
		},
	};
	let read_only = attrs.read_only;
	let range= &attrs.range;
	let mut tooltip = "";
	if let Some(ttip) = attrs.tooltip.as_ref() {
		tooltip = ttip;
	}
	if attrs.slider {
		if let Some(range) = range {
			let min = range.min;
			let max = range.max;
			let ty = proc_macro2::Ident::new(&get_path_str(&field.ty).unwrap(), proc_macro2::Span::call_site()); 
			return quote_spanned! {field.span() => {
					ui.scope(|ui| {
						//egui_inspect::InspectNumber::inspect_with_slider(#field_access, _parent_id, &#name_str, ui, #min, #max);
						<Self as egui_inspect::EguiInspect>::add_number_slider(#field_access, &#name_str, #tooltip, read_only || #read_only, #min as #ty, #max as #ty, ui);
					});
				}
			};
		} else {
			return quote_spanned! { field.span() => {compile_error!("range is mandatory with slider".into()); } };
		}
	} else if let Some(range) = range {
		let min = range.min;
		let max = range.max;
		let ty = proc_macro2::Ident::new(&get_path_str(&field.ty).unwrap(), proc_macro2::Span::call_site()); 
		return quote_spanned! {field.span() => {
				ui.scope(|ui| {
					<Self as egui_inspect::EguiInspect>::add_number(#field_access, &#name_str, #tooltip, read_only || #read_only, Some((#min as #ty, #max as #ty)), ui);
				});
			}
		};
	} else if let Some(multiline) = &attrs.multiline {
		let nb_lines = multiline.0;
		return quote_spanned! {
			field.span() => {
				ui.scope(|ui| {
					<Self as egui_inspect::EguiInspect>::add_string_multiline(#field_access, &#name_str, #tooltip, read_only || #read_only, #nb_lines, ui);
				});
			}
		};
	} else if attrs.color {
		return quote_spanned! {field.span() => {
				ui.scope(|ui| {
					//egui_inspect::InspectColor::inspect_color(#field_access, _parent_id, &#name_str, ui);
					<Self as egui_inspect::EguiInspect>::add_color(#field_access, &#name_str, #tooltip, read_only || #read_only, ui);
				});
			}
		};
	}

	quote_spanned! {
		field.span() => {
			let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
			let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
			ui.scope(|ui| {
				egui_inspect::EguiInspect::inspect_with_custom_id(#field_access, parent_id, &#name_str, #tooltip, read_only || #read_only, ui);
			});
		}
	}

}