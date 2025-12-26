use eframe::egui::{self, FontData, FontDefinitions, containers::menu::SubMenuButton};
use rustendulator_core::{Nes, RunMode};
use std::sync::Arc;

fn pixel_font_family() -> egui::FontFamily {
    egui::FontFamily::Name("pixel".into())
}

fn button_font_family() -> egui::FontFamily {
    egui::FontFamily::Name("button".into())
}

fn load_fonts(ctx: &egui::Context) {
    let mut font = FontDefinitions::default();

    font.font_data.insert(
        "press_start_2p".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/PressStart2P-Regular.ttf",
        )))),
    );

    font.font_data.insert(
        "venus_rising".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/fonts/venusris.ttf",
        )))),
    );

    font.families
        .insert(pixel_font_family(), vec!["press_start_2p".to_owned()]);

    font.families
        .insert(button_font_family(), vec!["venus_rising".to_owned()]);

    ctx.set_fonts(font);
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Rustendulator",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1280.0, 800.0)),
            ..eframe::NativeOptions::default()
        },
        Box::new(|cc| {
            load_fonts(&cc.egui_ctx);
            Ok(Box::new(Rustendulator::default()))
        }),
    )
}

struct Rustendulator {
    nes: Nes,
    show_left_panel: bool,
    show_right_panel: bool,
}

impl Default for Rustendulator {
    fn default() -> Self {
        Self {
            nes: Nes::new(),
            show_left_panel: true,
            show_right_panel: true,
        }
    }
}

impl eframe::App for Rustendulator {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Inputs

        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::U)) {
            self.show_left_panel = !self.show_left_panel;
        }

        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::P)) {
            self.show_right_panel = !self.show_right_panel;
        }

        // Menu

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
                    let label = if self.show_left_panel {
                        "Hide Left Panel"
                    } else {
                        "Show Left Panel"
                    };
                    if ui
                        .add(
                            egui::Button::new(label)
                                .shortcut_text("Ctrl+U")
                                .wrap_mode(egui::TextWrapMode::Extend),
                        )
                        .clicked()
                    {
                        self.show_left_panel = !self.show_left_panel;
                    };

                    let label = if self.show_right_panel {
                        "Hide Right Panel"
                    } else {
                        "Show Right Panel"
                    };
                    if ui
                        .add(
                            egui::Button::new(label)
                                .shortcut_text("Ctrl+P")
                                .wrap_mode(egui::TextWrapMode::Extend),
                        )
                        .clicked()
                    {
                        self.show_right_panel = !self.show_right_panel;
                    };

                    SubMenuButton::from_button(
                        egui::Button::new("Run Mode").right_text(SubMenuButton::RIGHT_ARROW),
                    )
                    .ui(ui, |ui| {
                        let current = self.nes.get_run_mode();

                        if ui
                            .add(
                                egui::Button::new("Paused")
                                    .shortcut_text("Ctrl+W")
                                    .selected(current == RunMode::Paused),
                            )
                            .clicked()
                        {
                            self.nes.set_run_mode(RunMode::Paused);
                        };

                        if ui
                            .add(
                                egui::Button::new("Running")
                                    .shortcut_text("Ctrl+R")
                                    .selected(current == RunMode::Running),
                            )
                            .clicked()
                        {
                            self.nes.set_run_mode(RunMode::Running);
                        };

                        if ui
                            .add(
                                egui::Button::new("Step Cycle")
                                    .shortcut_text("Ctrl+S")
                                    .selected(current == RunMode::StepCycle),
                            )
                            .clicked()
                        {
                            self.nes.set_run_mode(RunMode::StepCycle);
                        };

                        if ui
                            .add(
                                egui::Button::new("Step Instruction")
                                    .shortcut_text("Ctrl+I")
                                    .selected(current == RunMode::StepInstruction),
                            )
                            .clicked()
                        {
                            self.nes.set_run_mode(RunMode::StepInstruction);
                        };

                        if ui
                            .add(
                                egui::Button::new("Step Frame")
                                    .shortcut_text("Ctrl+F")
                                    .selected(current == RunMode::StepFrame),
                            )
                            .clicked()
                        {
                            self.nes.set_run_mode(RunMode::StepFrame);
                        };
                    })
                })
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .min_width(300.0)
            .show_animated(ctx, self.show_left_panel, |ui| {
                ui.style_mut().override_font_id =
                    Some(egui::FontId::new(12.0, pixel_font_family()));
                egui::TopBottomPanel::top("emulator_info").show_inside(ui, |ui| {
                    ui.heading("Emulator Info");
                    ui.scope(|ui| {
                        ui.style_mut().override_font_id =
                            Some(egui::FontId::new(12.0, button_font_family()));
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            let on = self.nes.is_powered_on();
                            let led_color = if on {
                                egui::Color32::from_rgb(220, 40, 40)
                            } else {
                                egui::Color32::from_gray(60)
                            };
                            let (rect, _resp) = ui
                                .allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                            ui.painter()
                                .rect_filled(rect, 0, egui::Color32::from_gray(60));
                            ui.painter().rect_filled(rect.shrink(2.0), 0.0, led_color);

                            let power_resp = ui
                                .allocate_ui_with_layout(
                                    egui::vec2(96.0, 40.0),
                                    egui::Layout::bottom_up(egui::Align::Center),
                                    |ui| {
                                        ui.add_sized(
                                            egui::vec2(96.0, 40.0),
                                            egui::Button::new(
                                                egui::RichText::new("\nPOWER")
                                                    .color(egui::Color32::from_rgb(220, 40, 40)),
                                            ),
                                        )
                                    },
                                )
                                .inner;
                            if power_resp.clicked() {
                                if self.nes.is_powered_on() {
                                    self.nes.power_off();
                                } else {
                                    self.nes.power_on();
                                }
                            }

                            let reset_resp = ui
                                .allocate_ui_with_layout(
                                    egui::vec2(96.0, 40.0),
                                    egui::Layout::bottom_up(egui::Align::Center),
                                    |ui| {
                                        ui.add_sized(
                                            egui::vec2(96.0, 40.0),
                                            egui::Button::new(
                                                egui::RichText::new("\nRESET")
                                                    .color(egui::Color32::from_rgb(220, 40, 40)),
                                            ),
                                        )
                                    },
                                )
                                .inner;
                            if reset_resp.clicked() {
                                if self.nes.is_powered_on() {
                                    self.nes.reset();
                                }
                            }
                        })
                    })
                });
                egui::TopBottomPanel::bottom("cpu_debug")
                    .show_inside(ui, |ui| ui.heading("CPU Debug"))
            });

        egui::SidePanel::right("PPU Debug").show_animated(ctx, self.show_right_panel, |ui| {
            ui.style_mut().override_font_id = Some(egui::FontId::new(12.0, pixel_font_family()));
            ui.heading("PPU Debug");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().override_font_id = Some(egui::FontId::new(12.0, pixel_font_family()));
            ui.heading("NES Display");
        });
    }
}
