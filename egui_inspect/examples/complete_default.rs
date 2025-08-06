
use egui_inspect::{DefaultEguiInspect, EGuiInspector};

use eframe::egui;

macro_rules! generate_struct_tuple {
	($name:ident, [$($ty:ty),*]) => {
		#[derive(DefaultEguiInspect, Debug, Default)]
		pub struct $name(
			$(
				$ty,
				#[inspect(range(min = 0., max = 12.0))]
				$ty,
				#[inspect(slider, range(min = "-10.", max = 12.0))]
				$ty,
			)*
		);
	};
}

/*macro_rules! generate_struct_named {
	($name:ident, [$($ty:ty => $label:ident),*]) => {
		#[derive(EguiInspect, Debug, Default)]
		pub struct $name {
			$(
				$label: $ty,
				#[inspect(range(min = 0., max = 12.0))]
				macro_metavar_expr_concat!($label, _range): $ty,
				$label##_range: $ty,
				#[inspect(slider, range(min = 0., max = 12.0))]
				$label##_slider: $ty,
			)*
		}
	};
}*/
/*macro_rules! generate_enum {
	($name:ident, $($n:literal),*) => {
		enum $name {
			$(Item$n),*
		}
	};
}*/
//generate_enum!(MyEnum, 0, 1, 2, 3, 4);
generate_struct_tuple!(StructTuple, [u8, u16, u32, u64, usize, f32, f64, String, &'static str]);
/*generate_struct_named!(StructNamed, [
	u8 => value_u8,
    u16 => value_u16,
    f32 => value_f32,
    f64 => value_f64
	]);*/


#[derive(Default, DefaultEguiInspect)]
struct MyApp {
	struct_tuple: StructTuple
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.add(EGuiInspector::new(self));
		});
	}
}

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Complete Example", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))));
}