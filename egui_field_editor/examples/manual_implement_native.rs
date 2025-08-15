

include!("shared/manual_implement.rs");

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Custom Implementation Example", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))));
}