#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winrs_app::app;

fn main() {
	app::run();
}
