
include!("shared/shared_data.rs");

fn main() {
	let options = eframe::NativeOptions::default();
	let _ = eframe::run_native("EGui Inspector Simple Example", options, 
		Box::new(|_cc|
			Ok(Box::new(MyApp::default()))
		)
	);
}