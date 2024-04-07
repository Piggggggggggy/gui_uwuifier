#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arboard::Clipboard;
use eframe::{
    egui::{self, Color32, FontFamily, FontId, Layout, Rounding, Widget},
    epaint::vec2,
};
use uwuifier::{round_up16, uwuify_sse};

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
        Box::new(|cc| {
            MyApp::new(
                cc,
                Clipboard::new().expect("could not get clipboard provider"),
            )
        }),
    )
}

struct MyApp {
    text: String,
    clipboard_provider: Clipboard,
}
impl MyApp {
    fn new(_cc: &eframe::CreationContext, provider: Clipboard) -> Box<Self> {
        let mut style = egui::Style::default();
        style.visuals.extreme_bg_color = Color32::from_hex("#0c0c0d").unwrap();
        style.visuals.override_text_color = Some(Color32::from_hex("#eef9ff").unwrap());
        style.visuals.panel_fill = Color32::from_hex("#0c0c0d").unwrap();
        _cc.egui_ctx.set_style(style);

        Box::new(Self {
            text: String::new(),
            clipboard_provider: provider,
        })
    }

    fn copy(&mut self) {
        self.clipboard_provider
            .set_text(self.text.clone())
            .expect("failed to set clipboard");
    }

    fn paste(&mut self) {
        if let Ok(text) = self.clipboard_provider.get_text() {
            self.text = text;
        }
    }

    fn sarcasm(&mut self) {
        let invert_case = |c: char| -> char {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                c.to_uppercase().next().unwrap()
            }
        };
        let result = self
            .text
            .chars()
            .map(|c| {
                if rand::random::<bool>() {
                    invert_case(c)
                } else {
                    c
                }
            })
            .collect::<String>();
        self.text = result;
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
        let default_button = |e: &str| {
            egui::Button::new(e)
                .min_size(vec2(70.0, 40.0))
                .rounding(Rounding::ZERO)
        };
        egui::TopBottomPanel::top("menu_bar")
            .min_height(40.0)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.style_mut().spacing.item_spacing.x = 0.0;
                    ui.horizontal(|ui: &mut egui::Ui| {
                        if default_button("uwuify").ui(ui).clicked() {
                            self.uwuify();
                        }
                        if default_button("sarcasm").ui(ui).clicked() {
                            self.sarcasm();
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
        egui::CentralPanel::default().show(ctx, |ui| {
            // main vertical layout
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.text)
                    .font(FontId::new(14.0, FontFamily::Proportional)),
            );
            // drag and drop text files
            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty() {
                    if let Some(path) = &i
                        .raw
                        .dropped_files
                        .clone()
                        .first()
                        .expect("should never fail")
                        .path
                    {
                        if let Ok(contents) = std::fs::read_to_string(path) {
                            self.text = contents;
                        }
                    }
                }
            })
        });
    }
}
