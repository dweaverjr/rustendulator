use eframe::egui;
use rustendulator_core::Nes;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Rustendulator",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.style_mut(|style| {
                style.animation_time = 0.5;
            });
            Ok(Box::new(Rustendulator::default()))
        }),
    )
}

struct Rustendulator {
    nes: Nes,
    show_cpu_debug: bool,
    show_ppu_debug: bool,
    show_memory_debug: bool,
}

impl Default for Rustendulator {
    fn default() -> Self {
        Self {
            nes: Nes::new(),
            show_cpu_debug: true,
            show_ppu_debug: true,
            show_memory_debug: true,
        }
    }
}

impl eframe::App for Rustendulator {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui
                        .add(egui::Button::new("Open ROM...").shortcut_text("Ctrl+O"))
                        .clicked()
                    {
                        // Load ROM
                    }
                    if ui
                        .add(egui::Button::new("Quit").shortcut_text("Ctrl+Q"))
                        .clicked()
                    {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Emulator", |ui| {
                    ui.checkbox(&mut self.show_cpu_debug, "CPU Debug");
                    ui.checkbox(&mut self.show_ppu_debug, "PPU Debug");
                    ui.checkbox(&mut self.show_memory_debug, "Memory Debug");
                })
            });
        });

        egui::SidePanel::left("cpu_debug").show_animated(ctx, self.show_cpu_debug, |ui| {
            ui.heading("CPU Debug");
        });

        egui::SidePanel::right("PPU Debug").show_animated(ctx, self.show_ppu_debug, |ui| {
            ui.heading("PPU Debug");
        });
        egui::TopBottomPanel::bottom("Memory Debug").show_animated(
            ctx,
            self.show_memory_debug,
            |ui| {
                ui.heading("Memory Debug");
            },
        );
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("NES Display");
        });
    }
}
