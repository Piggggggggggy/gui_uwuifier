#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::ops::RangeBounds;

use arboard::Clipboard;
use eframe::{
    egui::{self, Layout, Widget},
    epaint::vec2,
};
use regex::Regex;
use uwuifier::{round_up16, uwuify_sse, uwuify_str_sse};

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

    fn uwuify(&mut self) {
        let b = &self.text;
        // filter out urls
        let url_regex = regex::Regex::new(r"https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)").unwrap();
        let non_url_text = url_regex.split(b).collect::<Vec<_>>();
        let mut matches = url_regex.captures_iter(b).peekable();

        let mut temp1 = vec![0u8; round_up16(b.len()) * 16];
        let mut temp2 = vec![0u8; round_up16(b.len()) * 16];
        let mut text = non_url_text
            .iter()
            .map(|s| {
                String::from_utf8_lossy(uwuify_sse(s.as_bytes(), &mut temp1, &mut temp2))
                    .to_string()
            })
            .peekable();

        let mut result = String::new();
        while matches.peek().is_some() || text.peek().is_some() {
            if let Some(s) = text.next() {
                result.push_str(&s);
            };
            if let Some(s) = matches.next() {
                result.push_str(s.get(0).unwrap().as_str());
            };
        }
        self.text = result;
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let default_button = |e: &str| egui::Button::new(e).min_size(vec2(20.0, 30.0));
        egui::CentralPanel::default().show(ctx, |ui| {
            // main vertical layout
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
                            self.uwuify();
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
