#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use arboard::Clipboard;
use eframe::{
    egui::{self, Layout, Widget},
    epaint::vec2,
};
use uwuifier::uwuify_str_sse;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 600.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "Uwu",
        options,
        Box::new(|cc| MyApp::new(cc, Clipboard::new().unwrap())),
    )
}

struct MyApp {
    text: String,
    clipboard_provider: Clipboard,
}
impl MyApp {
    fn new(_cc: &eframe::CreationContext, provider: Clipboard) -> Box<Self> {
        Box::new(Self {
            text: String::new(),
            clipboard_provider: provider,
        })
    }

    fn copy(&mut self) {
        self.clipboard_provider.set_text(self.text.clone()).unwrap();
    }

    fn paste(&mut self) {
        if let Ok(text) = self.clipboard_provider.get_text() {
            self.text = text;
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let default_button = |e: &str| egui::Button::new(e).min_size(vec2(20.0, 30.0));
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let (_id, rect) = ui.allocate_space(vec2(ui.available_width(), 30.0));
                // text
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.text),
                );

                // control buttons
                ui.put(rect, |ui: &mut egui::Ui| {
                    ui.horizontal(|ui: &mut egui::Ui| {
                        if default_button("uwuify").ui(ui).clicked() {
                            self.text = uwuify_str_sse(&self.text);
                        }
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if default_button("Paste").ui(ui).clicked() {
                                    self.paste();
                                }
                                if default_button("Copy").ui(ui).clicked() {
                                    self.copy();
                                }
                            },
                        )
                    })
                    .response
                });
            });

            // drag and drop text files
            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty() {
                    if let Some(path) = &i.raw.dropped_files.clone().first().unwrap().path {
                        if let Ok(contents) = std::fs::read_to_string(path) {
                            self.text = contents;
                        }
                    }
                }
            })
        });
    }
}
