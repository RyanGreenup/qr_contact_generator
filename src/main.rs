#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

// Improve this so it can take a list of items AI!
struct SelectableList {
    selected_item: Option<usize>,
    item_open: [bool; 10],
}


impl SelectableList {
    fn new() -> Self {
        Self {
            selected_item: None,
            item_open: [false; 10],
        }
    }

    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Handle keyboard input
        if let Some(selected_item) = self.selected_item {
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                self.selected_item = Some((selected_item + 1).min(9));
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                self.selected_item = Some(selected_item.saturating_sub(1));
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                self.item_open[selected_item] = !self.item_open[selected_item];
            }
        }

        for i in 0..10 {
            let open = self.item_open[i];
            ui.collapsing(format!("List Item {}", i + 1), |ui| {
                if Some(i) == self.selected_item {
                    ui.visuals_mut().selection.bg_fill = egui::Color32::from_gray(196);
                }
                ui.label(format!("Sub-item {}-1", i + 1));
                ui.label(format!("Sub-item {}-2", i + 1));
                ui.label(format!("Sub-item {}-3", i + 1));
            });
            self.item_open[i] = open;
        }
    }
}

struct MyApp {
    name: String,
    age: u32,
    list: SelectableList,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            list: SelectableList::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            self.list.show(ctx, ui);

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}
