#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod chaos;

fn main() {
    let effect = chaos::Chaos::new();
    library::screensaver_runner::run_main(effect, "chaos");
}
