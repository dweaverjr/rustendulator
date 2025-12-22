use eframe::egui;
use rustendulator_core::Nes;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Rustendulator",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Rustendulator::default()))),
    )
}

struct Rustendulator {
    nes: Nes,
}

impl Default for Rustendulator {
    fn default() -> Self {
        Self { nes: Nes::new() }
    }
}

impl eframe::App for Rustendulator {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {}
}
