use rustendulator_core::Nes;

fn main() {
    let mut nes = Nes::new();

    nes.power_on();

    nes.run_frame();
}
