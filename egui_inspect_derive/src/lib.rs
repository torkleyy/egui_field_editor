use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
	parse_macro_input, parse_quote, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Generics, Index
};

use darling::{FromField, FromMeta, FromVariant};

mod utils;

#[derive(Debug, FromMeta)]
struct Range {
	#[darling(default)]
	min: f32,
	#[darling(default)]
	max: f32,
}
#[derive(Debug, FromField, FromVariant)]
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
	range: Option<Range>,
	/// Tooltip for the field
	tooltip: Option<String>
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
			range: None ,
			tooltip:None
		}
	}
}

#[proc_macro_derive(DefaultEguiInspect, attributes(inspect))]
pub fn derive_egui_inspect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	let generics = add_trait_bounds(input.generics);
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let inspect_code = get_code_for_data(&input.data, &name);

	let expanded = quote! {
		impl #impl_generics egui_inspect::DefaultEguiInspect for #name #ty_generics #where_clause {
			fn default_inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
				let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
				let parent_id = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
				#inspect_code
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

fn get_code_for_data(data: &Data, struct_name: &Ident) -> TokenStream {
	let ts = match *data {
		Data::Struct(ref data) => get_code_for_struct(&data),
		Data::Enum(ref an_enum) => get_code_for_enum(&struct_name, &an_enum),
		Data::Union(_) => unimplemented!("Unions are not supported (would need unsafe code)"),
	};
	ts
}

fn get_code_for_struct(data: &DataStruct)  -> TokenStream {
	match data.fields {
		Fields::Named(ref fields) => get_code_for_struct_named_fields(fields),
		Fields::Unnamed(ref fields) => get_code_for_struct_unnamed_fields(fields),
		Fields::Unit => quote! {}
	}
}
fn get_code_for_enum(enum_name: &Ident, data_enum: &DataEnum) -> TokenStream {
	let mut selected_text_arms = Vec::new();
	let mut selectable_blocks = Vec::new();
	let mut ui_match_arms = Vec::new();
	let mut has_hidden = false;

	for variant in &data_enum.variants {
		let variant_name = &variant.ident;
		let label = variant_name.to_string();
		let attr;
		match AttributeArgs::from_variant(variant) {
			Ok(_attr) => {
				attr=_attr;
			}
			Err(e) => {
				let ident = &variant.ident;
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
			continue;
		}
		match &variant.fields {
			Fields::Unit => get_code_for_unit_variant(enum_name, variant_name, label, &mut selected_text_arms, &mut selectable_blocks, &mut ui_match_arms),
			Fields::Unnamed(fields) => get_code_for_unamed_variant(enum_name, variant_name, label, fields, &mut selected_text_arms, &mut selectable_blocks, &mut ui_match_arms),
			Fields::Named(fields) => get_code_for_named_variant(enum_name, variant_name, label, fields, &mut selected_text_arms, &mut selectable_blocks, &mut ui_match_arms)

		}
	}
	
	if has_hidden {
		selected_text_arms.push(quote!{_ => {""}});
		ui_match_arms.push(quote! {_ => {} });
	}

	quote_spanned! {
		enum_name.span() => {
				let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
				//TODO: find a way to use it (if only Unit variants) or don't declare it if not needed
				#[allow(unused_variables)]
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
						.width(field_width)
						.selected_text(match self {
							#(#selected_text_arms)*
						})
						.show_ui(ui, |ui| {
							#(#selectable_blocks)*
						});
					});
				});

			match self {
				#(#ui_match_arms)*
			}
		}
	}
}

fn get_code_for_struct_named_fields(fields: &FieldsNamed) -> TokenStream {
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

		utils::get_function_call(quote!{&mut self.#name}, f, &attrs, "".into())
	});
	quote_spanned! {
		fields.span() => {
			let mut add_content=|ui:&mut egui::Ui| {
				#(#recurse)*
			};
			if !label.is_empty() {
				//ui.strong(label);
				ui.collapsing(label, add_content);
			} else {
				add_content(ui);
			}
		}
	}
}
fn get_code_for_struct_unnamed_fields(fields: &FieldsUnnamed) -> TokenStream {
	let mut recurse = Vec::new();
	for (i,f) in fields.unnamed.iter().enumerate() {
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
		let tuple_index = Index::from(i);
		recurse.push(utils::get_function_call(quote!{&mut self.#tuple_index}, f, &attr, format!("Field {i}")))
	};

	let result = quote_spanned! {
		fields.span() => {
			let mut add_content=|ui:&mut egui::Ui| {
				#(#recurse)*
			};
			if !label.is_empty() {
				ui.collapsing(label, add_content);
			} else {
				add_content(ui);
			}
		}
	};
	result
}

fn get_code_for_unit_variant(
		enum_name: &Ident,
		variant_name: &Ident,
		label:String,
		selected_text_arms:&mut Vec<TokenStream>,
		selectable_blocks:&mut Vec<TokenStream>,
		ui_match_arms:&mut Vec<TokenStream>) {
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

fn get_code_for_unamed_variant(
	enum_name: &Ident,
	variant_name: &Ident,
	label:String,
	fields : &FieldsUnnamed,
	selected_text_arms:&mut Vec<TokenStream>,
	selectable_blocks:&mut Vec<TokenStream>,
	ui_match_arms:&mut Vec<TokenStream>) {

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
			return quote!();
		}
		
		let fieldname = format!("field{}", i);
		let fieldname = Ident::new(&fieldname, proc_macro2::Span::call_site());
		fieldnames_list.push(quote!{#fieldname});

		utils::get_function_call(quote!{#fieldname}, f, &attr, format!("Field {i}"))
	});
	let bindings_for_match = bindings.clone();
	ui_match_arms.push(quote! {
		#enum_name::#variant_name(#(#bindings_for_match),* ) => {
			ui.indent(id, |ui| {
				#(#recurse)*
			});
		}
	});
}

fn get_code_for_named_variant(
		enum_name: &Ident,
		variant_name: &Ident,
		label:String,
		fields : &FieldsNamed,
		selected_text_arms:&mut Vec<TokenStream>,
		selectable_blocks:&mut Vec<TokenStream>,
		ui_match_arms:&mut Vec<TokenStream>) {

	let mut field_bindings = Vec::new();
	let mut inspect_calls = Vec::new();
	
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

	for f in &fields.named {
		let fieldname = f.ident.as_ref().unwrap();
		match AttributeArgs::from_field(f) {
			Ok(attr) => {
				if attr.hidden {
					continue;
				}

				inspect_calls.push(utils::get_function_call(quote!{#fieldname}, f, &attr, "".into()));
			}
			Err(e) => {
				let msg = e.to_string();
				inspect_calls.push(quote! {
					compile_error!(#msg);
				});
			}
		}
		field_bindings.push(fieldname);
	}

	ui_match_arms.push(quote! {
		#enum_name::#variant_name { #( #field_bindings ),* } => {
			ui.indent(id, |ui| {
				#( #inspect_calls )*
			});
		}
	});
}