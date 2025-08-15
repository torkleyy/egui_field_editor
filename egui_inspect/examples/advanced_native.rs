
include!("advanced.rs");

fn main() {
{
	let app = MyApp::default();
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Advanced Example", options, Box::new(|_cc| Ok(Box::new(app))));
}}

