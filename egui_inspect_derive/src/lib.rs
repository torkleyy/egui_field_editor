use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
	parse_macro_input, parse_quote, spanned::Spanned, Data, DataEnum, DeriveInput, Fields, FieldsNamed, GenericParam, Generics, Index
};

use darling::{FromField, FromMeta};

mod internal_paths;
mod utils;

#[derive(Debug, FromMeta)]
struct Range {
	#[darling(default)]
	min: f32,
	#[darling(default)]
	max: f32,
}
#[derive(Debug, FromField)]
#[darling(attributes(inspect), default)]
struct AttributeArgs {
	/// Name of the field to be displayed on UI labels
	name: Option<String>,
	/// Doesn't generate code for the given field
	hidden: bool,
	/// Doesn't call mut function for the given field (May be overridden by other params)
	read_only: bool,
	/// Use slider function for numbers
	slider: bool,
	/// Display mut text on multiple line
	multiline: bool,
	/// Display mut vec3/vec4 with color
	color: bool,
	/// Min/Max values for numbers
	range: Option<Range>
}

impl Default for AttributeArgs {
	fn default() -> Self {
		Self {
			name: None,
			hidden: false,
			read_only: false,
			slider: false,
			multiline: false,
			color: false,
			range: None 
		}
	}
}

#[proc_macro_derive(DefaultEguiInspect, attributes(inspect))]
pub fn derive_egui_inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	let generics = add_trait_bounds(input.generics);
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let inspect_mut = inspect_struct(&input.data, &name);

	let expanded = quote! {
		impl #impl_generics egui_inspect::EguiInspect for #name #ty_generics #where_clause {
			fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, ui: &mut egui::Ui) {
				#inspect_mut
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
	for param in &mut generics.params {
		if let GenericParam::Type(ref mut type_param) = *param {
			type_param
				.bounds
				.push(parse_quote!(egui_inspect::EguiInspect));
		}
	}
	generics
}

fn inspect_struct(data: &Data, _struct_name: &Ident) -> TokenStream {
	let ts = match *data {
		Data::Struct(ref data) => match data.fields {
			Fields::Named(ref fields) => handle_named_fields(fields),
			Fields::Unnamed(ref fields) => {
				let mut recurse = Vec::new();
				for (i,f) in fields.unnamed.iter().enumerate() {
					let tuple_index = Index::from(i);
					let name = format!("Field {i}");
					recurse.push(quote_spanned! { f.span() => egui_inspect::EguiInspect::inspect_with_custom_id(&mut self.#tuple_index, _parent_id.with(label), #name, ui);});
				};

				let result = quote_spanned! {
					fields.span() => {
						ui.strong(label);
						#(#recurse)*
					}
				};
				result
			},
			Fields::Unit => quote! {}
		},
		Data::Enum(ref an_enum) => handle_enum(&_struct_name, &an_enum),
		Data::Union(_) => unimplemented!("Unions are not supported (would need unsafe code)"),
	};
	ts
}


fn handle_enum(enum_name: &Ident, data_enum: &DataEnum) -> TokenStream {
	let mut selected_text_arms = Vec::new();
	let mut selectable_blocks = Vec::new();
	let mut ui_match_arms = Vec::new();

	for variant in &data_enum.variants {
		let variant_name = &variant.ident;
		let label = variant_name.to_string();

		match &variant.fields {
			Fields::Unit => {
				selected_text_arms.push(quote! {
					#enum_name::#variant_name => #label,
				});

				selectable_blocks.push(quote! {
					if ui.selectable_value(self, #enum_name::#variant_name, #label).changed() {
						*self = #enum_name::#variant_name;
					}
				});


				ui_match_arms.push(quote! {
					#enum_name::#variant_name => {
						// nothing to edit
					}
				});
			}

			Fields::Unnamed(fields) => {
				let default_value = if fields.unnamed.len() == 1 {
					quote! { Default::default() }
				} else {
					let defaults = std::iter::repeat(quote! { Default::default() })
						.take(fields.unnamed.len());
					quote! {  #(#defaults),*  }
				};
				let bindings_ignore = (0..fields.unnamed.len())
					.map(|_i| Ident::new(&"_", proc_macro2::Span::call_site()));
				selected_text_arms.push(quote! {
					#enum_name::#variant_name(#(#bindings_ignore),*) => #label,
				});

				selectable_blocks.push(quote! {
					if ui.selectable_value(self, #enum_name::#variant_name(#default_value), #label).changed() {
						*self = #enum_name::#variant_name(#default_value);
					}
				});
				let mut has_hidden = false;
				let mut fieldnames_list = vec![];
				let bindings = (0..fields.unnamed.len())
					.map(|i| Ident::new(&format!("field{}", i), proc_macro2::Span::call_site()));
				
				let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
					let attr;
					match AttributeArgs::from_field(f) {
						Ok(_attr) => {
							attr=_attr;
						}
						Err(e) => {
							let ident = &f.ident;
							let msg = e.to_string();
							return quote! {
								#ident: {
									compile_error!(#msg);
								}
							};
						}
					}
					if attr.hidden {
						has_hidden = true;
						return quote!();
					}
					
					let fieldname = format!("field{}", i);
					let fieldname = Ident::new(&fieldname, proc_macro2::Span::call_site());
					fieldnames_list.push(quote!{#fieldname});

					//let bindings_for_match = bindings.clone();
    				if let Some(ts) = internal_paths::try_handle_internal_path(quote!{#fieldname}, f, &attr, format!("Field {i}")) {
						//return quote! { #enum_name::#variant_name(#(#bindings_for_match),*) => #ts };
						return ts;
					}
					quote! {
						//#enum_name::#variant_name(#(#bindings_for_match),* ) => {
							egui_inspect::EguiInspect::inspect_with_custom_id(#fieldname, _parent_id.with(label), label, ui);
						//}
					}
				});
				/*let bindings_for_match = bindings.clone();
				println!("{}", quote! {
					#enum_name::#variant_name(#(#bindings_for_match),* ) => {
						#(#recurse)*
					}
				});*/
    			let bindings_for_match = bindings.clone();
				ui_match_arms.push(quote! {
					#enum_name::#variant_name(#(#bindings_for_match),* ) => {
						#(#recurse)*
					}
				});
				if has_hidden {
					ui_match_arms.push(quote! {_ => {} });
				}
			}

			Fields::Named(fields) => {
				let mut field_bindings = Vec::new();
				let mut inspect_calls = Vec::new();
				let mut has_hidden = false;
				
				let bindings_ignore: Vec<TokenStream> = fields.named.iter()
					.map(|f| {
						let name = f.ident.clone();
						quote!{#name : _ }
					}
				).collect();
				selected_text_arms.push(quote! {
					#enum_name::#variant_name{#(#bindings_ignore),*} => #label,
				});
				
				let defaults = fields.named.iter().filter_map(|field| {
					let name = field.ident.as_ref().unwrap(); // ignore les champs sans identifiant
					Some(quote! { #name: Default::default() })
				}).collect::<Vec<_>>();
				let default_value = quote! {  #(#defaults),* };

				selectable_blocks.push(quote! {
					if ui.selectable_value(self, #enum_name::#variant_name{#default_value}, #label).changed() {
						*self = #enum_name::#variant_name { #default_value };
					}
				});

				for field in &fields.named {
					let field_name = field.ident.as_ref().unwrap();
					match AttributeArgs::from_field(field) {
						Ok(attr) => {
							if attr.hidden {
								has_hidden = true;
								continue;
							}

							inspect_calls.push(quote! {
								egui_inspect::EguiInspect::inspect_with_custom_id(
									#field_name,
									_parent_id.with(stringify!(#field_name)),
									stringify!(#field_name),
									ui
								);
							});
						}
						Err(e) => {
							let msg = e.to_string();
							inspect_calls.push(quote! {
								compile_error!(#msg);
							});
						}
					}
					field_bindings.push(field_name);
				}

				ui_match_arms.push(quote! {
					#enum_name::#variant_name { #( #field_bindings ),* } => {
						#( #inspect_calls )*
					}
				});

				if has_hidden {
					ui_match_arms.push(quote! { _ => {} });
				}
			}

		}
	}

	quote_spanned! {
		enum_name.span() => {
			egui::ComboBox::from_id_salt(ui.next_auto_id())
				.selected_text(match self {
					#(#selected_text_arms)*
				})
				.show_ui(ui, |ui| {
					#(#selectable_blocks)*
				});

			match self {
				#(#ui_match_arms)*
			}
		}
	}
}

fn handle_named_fields(fields: &FieldsNamed) -> TokenStream {
	let recurse = fields.named.iter().map(|f| {
		let attrs;
					match AttributeArgs::from_field(f) {
						Ok(_attr) => {
							attrs=_attr;
						}
						Err(e) => {
							let ident = &f.ident;
							let msg = e.to_string();
							return quote_spanned! { ident.span() => {
									compile_error!(#msg);
								}
							};
						}
					}
		if attrs.hidden {
			return quote!();
		}
		let name = &f.ident;
		if let Some(ts) = internal_paths::try_handle_internal_path(quote!{&mut self.#name}, f, &attrs,"".into()) {
			return ts;
		}

		utils::get_default_function_call(f, &attrs, "".into())
	});
	quote_spanned! {
		fields.span() => {
			ui.strong(label);
			#(#recurse)*
		}
	}
}
