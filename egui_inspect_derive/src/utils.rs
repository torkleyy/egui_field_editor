use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::Type::{Path, Reference};
use syn::{Field, Type};

use crate::AttributeArgs;

pub fn get_path_str(type_path: &Type) -> Option<String> {
	match type_path {
		Path(type_path) => {
			let ident = type_path
				.path
				.get_ident();
			ident.map(|name| name.to_string())
		}
		Reference(type_ref) => get_path_str(&type_ref.elem),
		_ => Some("".to_string()),
	}
}

pub(crate) fn get_default_function_call(field: &Field, attrs: &AttributeArgs, default_field_name:String) -> TokenStream {
	let name = &field.ident;

	let name_str = match &attrs.name {
		Some(n) => n.clone(),
		None => {
			if let Some(name) = name {
				name.to_string()
			} else {
				default_field_name
			}
		},
	};


	quote_spanned! {field.span() => {egui_inspect::EguiInspect::inspect_with_custom_id(&mut self.#name, _parent_id.with(label), &#name_str, ui);}}

}