# egui_inspect
![crates.io](https://img.shields.io/crates/l/egui_inspect.svg)

This crate is intended to provide some rust helper macros to automatically generate boilerplate code to inspect
structures

Its goals are:

- to provide as much compile-time generated code as possible, avoiding conditional branches at runtime
- to be hyper user-friendly

This crate provide a `EguiInspect` trait which is necessary for a struct or enum to be inspected. This trait is implemented for many base
types, and can be implemented for user created types with the macro `#[derive(DefaultEguiInspect)]` (see [here](#customizing-generated-code) to know why `Default`).
If every underlying types implements `EguiInspect`, then you will be able to inspect it.

You optionally can add a `nalgebra_glm` support which provide implementation of `EguiInspect` for `nalgebra_glm` types.

This is a side project, at a very early state, so the API might not be stable yet.

# Example

![img.png](resources/screenshot.png)


To implement this example, you just need to add egui_inspect as dependency to your project, and then, when drawing you
ui with egui, you need to give your `&Ui` to the inspect function, no need for additional input .
See the following examples:
 * [simple](egui_inspect/examples/simple.rs): a simple example
 * [nalgebra_glm](egui_inspect/examples/nalgebra_glm.rs): example with `nalgebra_glm` types.
 * [complete](egui_inspect/examples/complete_default.rs): example aiming to be a test case by displaying all supported types/enum/structs.


# Documentation
## Customizing generated code
This will generate a `DefaultEguiInspect` implementation and a blanked implementation of `EguiInspect` is implemented for types implementing `DefaultEguiInspect`. That way, you can directly use your type to inspect it or you can implement `EguiInspect` and use the default implementation if needed.

For exemple:
```rust
#[derive(DefaultEguiInspect)]
struct MyStruct {
	is_complex_ui:bool,
	#[inspect(hidden)]
	complex_ui_needed:MyStructOrEnumNeedingComplexUI,
	simple_ui_needed:MyStructOrEnumNeedingSimpleUI,
}
impl EguiInspect for MyStruct {
	fn inspect_with_custom_id(&mut self, parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		if self.is_complex_ui {
			// implement the complex ui
		} else {
			self.default_inspect_with_custom_id(self, parent_id, label, tooltip, read_only, ui);
		}
	}
}
```

## Implement `EguiInspect` yourself
The trait `EguiInspect` provides many default implementation of functions to edit basic types. So implementing in simple case is pretty straightforward.

For example:
```rust
struct MyStruct {
	a_bool:bool,
	an_int:i32,
	an_uint:u64,
	a_float:f32,
	a_color:egui::Color32,
	a_string:String,
	a_second_string:String,
}
impl egui_inspect::EguiInspect for MyStruct {
	fn inspect_with_custom_id(&mut self, _parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui) {
		let id = if _parent_id == egui::Id::NULL { ui.next_auto_id() } else { _parent_id.with(label) };
		let _parent_id_to_provide_to_children = if _parent_id == egui::Id::NULL { egui::Id::NULL } else { id };
		let mut add_content=|ui:&mut egui::Ui| {
			Self::add_bool(&mut self.a_bool, "Bool", "Boolean Tooltip", read_only, ui);
			Self::add_number(&mut self.an_int, "Integer", "Integer Tooltip", read_only, None, ui);
			Self::add_number(&mut self.an_uint, "Unsigned Integer", "Unsigned Integer Tooltip with min/max", read_only, Some((12, 50000)), ui);
			Self::add_number_slider(&mut self.a_float, "Float", "Float Slider Tooltip", read_only, -12., 50., ui);
			Self::add_color(&mut self.a_color, "Color", "", read_only, ui);
			Self::add_string_singleline(&mut self.a_string, "String", "", read_only, ui);
			Self::add_string_multiline(&mut self.a_second_string, "Multiline String", "", read_only, ui);
		};
		if !label.is_empty() {
			egui::CollapsingHeader::new(label).id_salt(id).show(ui, add_content);
		} else {
			add_content(ui);
		}
	}
}
```

## Why 2 inspect methods ?
The trait `EguiInspect` provide two methods :
 * `fn inspect(&mut self, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui);`
 * `fn inspect_with_custom_id(&mut self, parent_id: egui::Id, label: &str, tooltip: &str, read_only: bool, ui: &mut egui::Ui);`

The first method is a convenience wrapper: it’s implemented by default and simply calls the second one using parent_id = Id::NULL.

### So Why Have Both?

Egui internally tracks widget state—such as whether a collapsible section is open, the scroll position, and other UI dynamics. To do this reliably, widgets that maintain state need a unique `Id`.
By default, egui generates these IDs automatically. However, this can lead to inconsistencies when the UI structure changes dynamically, especially if the auto-generated IDs don’t remain stable across frames.
That’s where `inspect_with_custom_id` comes in. It allows you to explicitly pass a `parent_id`, which helps maintain consistent and conflict-free IDs.

### How It Works
When `inspect_with_custom_id` is called, the implementation combines the parent_id with the widget’s label to generate a unique ID. This composite `Id` is then used for the widget itself and passed down to its children. This hierarchical `Id` scheme helps avoid `Id` collisions (at least, I hope) and should ensures stable widget behavior even when the UI changes.


