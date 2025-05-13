#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use qrcode_generator::QrCodeEcc;
use std::path::PathBuf;

/// Represents business contact information for vCard generation
#[derive(Default)]
pub struct BusinessContact {
    pub first_name: String,
    pub last_name: String,
    pub organization: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub mobile: String,
    pub website: String,
    pub address: String,
    pub note: String,
}

impl BusinessContact {
    /// Generates a vCard string from the business contact information
    pub fn generate_vcard(&self) -> String {
        let mut vcard = String::new();

        vcard.push_str("BEGIN:VCARD\n");
        vcard.push_str("VERSION:3.0\n");
        vcard.push_str(&format!("N:{};{};;;\n", self.last_name, self.first_name));
        vcard.push_str(&format!("FN:{} {}\n", self.first_name, self.last_name));

        if !self.organization.is_empty() {
            vcard.push_str(&format!("ORG:{}\n", self.organization));
        }

        if !self.title.is_empty() {
            vcard.push_str(&format!("TITLE:{}\n", self.title));
        }

        if !self.email.is_empty() {
            vcard.push_str(&format!("EMAIL;type=WORK,INTERNET:{}\n", self.email));
        }

        if !self.phone.is_empty() {
            vcard.push_str(&format!("TEL;type=WORK,voice:{}\n", self.phone));
        }

        if !self.mobile.is_empty() {
            vcard.push_str(&format!("TEL;type=CELL,voice:{}\n", self.mobile));
        }

        if !self.website.is_empty() {
            vcard.push_str(&format!("URL:{}\n", self.website));
        }

        if !self.address.is_empty() {
            vcard.push_str(&format!("ADR;type=WORK:;;{};;;;\n", self.address));
        }

        if !self.note.is_empty() {
            vcard.push_str(&format!("NOTE:{}\n", self.note));
        }

        vcard.push_str("END:VCARD");
        vcard
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Business Card QR Generator",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<BusinessCardApp>::default())
        }),
    )
}

struct BusinessCardApp {
    contact: BusinessContact,
    vcard_text: String,
    show_copied_toast: bool,
    toast_time: f32,
    qr_code_texture: Option<egui::TextureHandle>,
    show_saved_toast: bool,
    saved_toast_time: f32,
    save_path: String,
}

impl Default for BusinessCardApp {
    fn default() -> Self {
        Self {
            contact: BusinessContact::default(),
            vcard_text: String::new(),
            show_copied_toast: false,
            toast_time: 0.0,
            qr_code_texture: None,
            show_saved_toast: false,
            saved_toast_time: 0.0,
            save_path: String::from("qrcode.png"),
        }
    }
}

impl BusinessCardApp {
    // Generate QR code image from a string and convert to ColorImage for egui
    fn generate_qr_code(&self, text: &str) -> egui::ColorImage {
        // Generate QR code with medium error correction
        let qr_code = qrcode_generator::to_image_buffer(text, QrCodeEcc::Medium, 512).unwrap();

        // Get dimensions
        let width = qr_code.width() as usize;
        let height = qr_code.height() as usize;

        // Convert from grayscale to RGBA
        let mut rgba_data = Vec::with_capacity(width * height * 4);

        for pixel in qr_code.pixels() {
            // QR codes are black (0) and white (255)
            let value = pixel[0];

            // Black pixels (value = 0) become black (0, 0, 0, 255)
            // White pixels (value = 255) become white (255, 255, 255, 255)
            rgba_data.push(value); // R
            rgba_data.push(value); // G
            rgba_data.push(value); // B
            rgba_data.push(255); // A (always fully opaque)
        }

        egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba_data)
    }

    // Save QR code as a PNG file
    fn save_qr_code_to_png(&self, path: &str) -> Result<(), String> {
        if self.vcard_text.is_empty() {
            return Err("No vCard generated yet".to_string());
        }

        // Generate QR code directly using qrcode-generator
        let qr_code =
            qrcode_generator::to_image_buffer(&self.vcard_text, QrCodeEcc::Medium, 512).unwrap();

        // Save image to file
        let file_path = PathBuf::from(path);
        match qr_code.save(file_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to save QR code: {}", e)),
        }
    }
}

impl eframe::App for BusinessCardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::G)) {
            // Ctrl+G to generate vCard
            self.vcard_text = self.contact.generate_vcard();

            // Generate QR code when vCard is generated
            if !self.vcard_text.is_empty() {
                let color_image = self.generate_qr_code(&self.vcard_text);

                // Load or update texture
                self.qr_code_texture =
                    Some(ctx.load_texture("qr-code", color_image, Default::default()));
            }
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::C))
            && !self.vcard_text.is_empty()
        {
            // Ctrl+C to copy vCard to clipboard (when vCard exists)
            ctx.output_mut(|o| o.copied_text = self.vcard_text.clone());
            self.show_copied_toast = true;
            self.toast_time = 0.0;
        }

        // Ctrl+S to save/download QR code as PNG
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S))
            && self.qr_code_texture.is_some()
            && !self.vcard_text.is_empty()
        {
            println!("Saving QR code to file: {}", self.save_path);
            match self.save_qr_code_to_png(&self.save_path) {
                Ok(_) => {
                    self.show_saved_toast = true;
                    self.saved_toast_time = 0.0;
                    println!("QR code saved successfully to {}", self.save_path);
                }
                Err(e) => {
                    eprintln!("Error saving QR code: {}", e);
                }
            }
        }

        // Update toast timers
        if self.show_copied_toast {
            let delta = ctx.input(|i| i.stable_dt);
            self.toast_time += delta;
            if self.toast_time > 2.0 {
                // Show toast for 2 seconds
                self.show_copied_toast = false;
            }
        }

        if self.show_saved_toast {
            let delta = ctx.input(|i| i.stable_dt);
            self.saved_toast_time += delta;
            if self.saved_toast_time > 2.0 {
                // Show toast for 2 seconds
                self.show_saved_toast = false;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Business Card QR Generator");

            ui.spacing_mut().item_spacing = egui::vec2(0.0, 10.0);

            // Contact information form
            ui.group(|ui| {
                ui.heading("Contact Information");

                ui.columns(2, |columns| {
                    // Left column
                    columns[0].vertical(|ui| {
                        ui.add_space(5.0);
                        ui.label("First Name:");
                        ui.text_edit_singleline(&mut self.contact.first_name);

                        ui.add_space(5.0);
                        ui.label("Last Name:");
                        ui.text_edit_singleline(&mut self.contact.last_name);

                        ui.add_space(5.0);
                        ui.label("Organization:");
                        ui.text_edit_singleline(&mut self.contact.organization);

                        ui.add_space(5.0);
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut self.contact.title);

                        ui.add_space(5.0);
                        ui.label("Email:");
                        ui.text_edit_singleline(&mut self.contact.email);
                    });

                    // Right column
                    columns[1].vertical(|ui| {
                        ui.add_space(5.0);
                        ui.label("Phone:");
                        ui.text_edit_singleline(&mut self.contact.phone);

                        ui.add_space(5.0);
                        ui.label("Mobile:");
                        ui.text_edit_singleline(&mut self.contact.mobile);

                        ui.add_space(5.0);
                        ui.label("Website:");
                        ui.text_edit_singleline(&mut self.contact.website);

                        ui.add_space(5.0);
                        ui.label("Address:");
                        ui.text_edit_singleline(&mut self.contact.address);

                        ui.add_space(5.0);
                        ui.label("Note:");
                        ui.text_edit_singleline(&mut self.contact.note);
                    });
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Generate vCard (Ctrl+G)").clicked() {
                        self.vcard_text = self.contact.generate_vcard();

                        // Generate QR code when vCard is generated
                        if !self.vcard_text.is_empty() {
                            let color_image = self.generate_qr_code(&self.vcard_text);

                            // Load or update texture
                            self.qr_code_texture =
                                Some(ctx.load_texture("qr-code", color_image, Default::default()));
                        }
                    }
                });
            });

            ui.add_space(10.0);

            // Display vCard and future QR code
            ui.group(|ui| {
                ui.heading("Generated vCard");

                ui.columns(2, |columns| {
                    // vCard text
                    columns[0].vertical(|ui| {
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.label("vCard Content:");
                            if ui.button("Copy to Clipboard (Ctrl+C)").clicked()
                                && !self.vcard_text.is_empty()
                            {
                                ui.output_mut(|o| o.copied_text = self.vcard_text.clone());
                                self.show_copied_toast = true;
                                self.toast_time = 0.0;
                            }
                        });
                        ui.add(
                            egui::TextEdit::multiline(&mut self.vcard_text)
                                .desired_width(f32::INFINITY)
                                .desired_rows(10)
                                .lock_focus(true)
                                .interactive(false),
                        );
                    });

                    // QR code image
                    columns[1].vertical(|ui| {
                        ui.add_space(5.0);
                        ui.heading("QR Code");

                        if let Some(texture) = &self.qr_code_texture {
                            // Display the QR code image
                            let size = 200.0;
                            let image = egui::Image::new(texture)
                                .fit_to_exact_size(egui::vec2(size, size))
                                .bg_fill(egui::Color32::WHITE);

                            ui.centered_and_justified(|ui| {
                                ui.add(image);
                            });

                            // Add a button to save QR code
                            ui.add_space(10.0);
                            ui.vertical_centered(|ui| {
                                ui.label("Save QR Code:");
                                ui.horizontal(|ui| {
                                    ui.label("Filename:");
                                    ui.add_sized(
                                        egui::vec2(200.0, 24.0),
                                        egui::TextEdit::singleline(&mut self.save_path),
                                    );

                                    // Make the download button more noticeable
                                    let download_button =
                                        egui::Button::new("📥 Download PNG (Ctrl+S)")
                                            .min_size(egui::vec2(180.0, 28.0))
                                            .fill(egui::Color32::from_rgb(100, 200, 100));

                                    // Add the button with tooltip
                                    let response = ui.add(download_button).on_hover_text(
                                        "Download the QR code as a PNG file\nHotkey: Ctrl+S",
                                    );

                                    if response.clicked() {
                                        println!("Saving QR code to file: {}", self.save_path);
                                        match self.save_qr_code_to_png(&self.save_path) {
                                            Ok(_) => {
                                                self.show_saved_toast = true;
                                                self.saved_toast_time = 0.0;
                                                println!(
                                                    "QR code saved successfully to {}",
                                                    self.save_path
                                                );
                                            }
                                            Err(e) => {
                                                eprintln!("Error saving QR code: {}", e);
                                            }
                                        }
                                    }
                                });
                            });
                        } else {
                            // Draw placeholder
                            let qr_rect = egui::Rect::from_min_size(
                                ui.cursor().min,
                                egui::vec2(150.0, 150.0),
                            );
                            ui.allocate_rect(qr_rect, egui::Sense::hover());
                            ui.painter().rect_stroke(
                                qr_rect,
                                0.0,
                                egui::Stroke::new(1.0, egui::Color32::GRAY),
                            );
                            ui.add_space(150.0);
                            ui.centered_and_justified(|ui| {
                                ui.label("QR Code will appear here");
                            });
                        }
                    });
                });
            });

            // Display toast notifications
            // Toast for clipboard copy
            if self.show_copied_toast {
                let screen_rect = ctx.screen_rect();
                let toast_rect = egui::Rect::from_center_size(
                    egui::pos2(screen_rect.center().x, screen_rect.max.y - 40.0),
                    egui::vec2(200.0, 30.0),
                );

                let painter = ctx.layer_painter(egui::LayerId::new(
                    egui::Order::Foreground,
                    egui::Id::new("copied_toast"),
                ));

                painter.rect_filled(
                    toast_rect,
                    5.0,
                    egui::Color32::from_rgba_premultiplied(20, 20, 20, 200),
                );

                painter.text(
                    toast_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "vCard copied to clipboard!",
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );
            }

            // Toast for QR code saved
            if self.show_saved_toast {
                let screen_rect = ctx.screen_rect();
                let toast_rect = egui::Rect::from_center_size(
                    egui::pos2(screen_rect.center().x, screen_rect.max.y - 40.0),
                    egui::vec2(200.0, 30.0),
                );

                let painter = ctx.layer_painter(egui::LayerId::new(
                    egui::Order::Foreground,
                    egui::Id::new("saved_toast"),
                ));

                painter.rect_filled(
                    toast_rect,
                    5.0,
                    egui::Color32::from_rgba_premultiplied(20, 20, 20, 200),
                );

                painter.text(
                    toast_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("📥 QR code downloaded as {}", self.save_path),
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );
            }
        });
    }
}
