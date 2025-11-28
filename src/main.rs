use rustendulator::Nes;

fn main() {
    let mut nes = Nes::new();

    nes.run_frame();
}
