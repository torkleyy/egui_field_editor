use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::Type;
use syn::{Field};

use crate::AttributeArgs;

pub fn get_path_str(type_path: &Type) -> Option<String> {
	match type_path {
		Type::Path(type_path) => {
			let ident = type_path
				.path
				.get_ident();
			ident.map(|name| name.to_string())
		}
		Type::Reference(type_ref) => get_path_str(&type_ref.elem),
		Type::Array(type_array) => get_path_str(&type_array.elem),
		//Type::BareFn(type_bare_fn)  => get_path_str(&type_bare_fn.elem),
		Type::Group(type_group) => get_path_str(&type_group.elem),
		//Type::ImplTrait(type_impl_trait) => get_path_str(&type_impl_trait.elem),
		//Type::Infer(type_infer) => get_path_str(&type_infer.elem),
		//Type::Macro(type_macro) => get_path_str(&type_macro.elem),
		//Type::Never(type_never) => get_path_str(&type_never.elem),
		Type::Paren(type_paren) => get_path_str(&type_paren.elem),
		Type::Ptr(type_ptr) => get_path_str(&type_ptr.elem),
		Type::Slice(type_slice) => get_path_str(&type_slice.elem),
		//Type::TraitObject(type_trait_object) => get_path_str(&type_trait_object.elem),
		//Type::Tuple(type_tuple) => get_path_str(&type_tuple.elem),
		//Type::Verbatim(token_stream) => get_path_str(&token_stream.elem),
		_ => {
			//println!("No path {:?}", type_path);
			Some("".to_string())
		},
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