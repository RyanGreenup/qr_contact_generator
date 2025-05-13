#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

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
}

impl Default for BusinessCardApp {
    fn default() -> Self {
        Self {
            contact: BusinessContact::default(),
            vcard_text: String::new(),
        }
    }
}

impl eframe::App for BusinessCardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                if ui.button("Generate vCard").clicked() {
                    self.vcard_text = self.contact.generate_vcard();
                }
            });
            
            ui.add_space(10.0);
            
            // Display vCard and future QR code
            ui.group(|ui| {
                ui.heading("Generated vCard");
                
                ui.columns(2, |columns| {
                    // vCard text
                    columns[0].vertical(|ui| {
                        ui.add_space(5.0);
                        ui.label("vCard Content:");
                        ui.add(egui::TextEdit::multiline(&mut self.vcard_text)
                            .desired_width(f32::INFINITY)
                            .desired_rows(10)
                            .lock_focus(true)
                            .interactive(false));
                    });
                    
                    // Placeholder for QR code image
                    columns[1].vertical(|ui| {
                        ui.add_space(5.0);
                        ui.heading("QR Code");
                        let qr_rect = egui::Rect::from_min_size(
                            ui.cursor().min, 
                            egui::vec2(150.0, 150.0),
                        );
                        ui.allocate_rect(qr_rect, egui::Sense::hover());
                        ui.painter().rect_stroke(
                            qr_rect, 
                            0.0, 
                            egui::Stroke::new(1.0, egui::Color32::GRAY)
                        );
                        ui.add_space(150.0);
                        ui.centered_and_justified(|ui| {
                            ui.label("QR Code will appear here");
                        });
                    });
                });
            });
        });
    }
}
