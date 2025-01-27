# Building a Simple List Widget in Egui

This tutorial shows how to build a list widget with keyboard navigation and collapsible items in Egui.

## The Core Structure

The list widget needs three pieces of state:
1. The items to display
2. Which item is selected
3. Which items are expanded/collapsed

Here's the basic structure:

```rust
struct SelectableList<T> {
    items: Vec<T>,
    selected_item: Option<usize>,
    item_open: Vec<bool>,
}
```

## Making it Generic

The list should work with any type that can be displayed. We use a generic type T that implements Display:

```rust
impl<T: std::fmt::Display> SelectableList<T> {
    fn new(items: Vec<T>) -> Self {
        let len = items.len();
        Self {
            items,
            selected_item: None,
            item_open: vec![false; len],
        }
    }
}
```

## Adding Keyboard Navigation

The widget handles three keyboard actions:

- Up arrow moves selection up
- Down arrow moves selection down
- Space toggles item expansion
    - Tab is the default to move between widgets, just like PySide6.

The `saturating_sub()` method performs subtraction that saturates at zero instead of underflowing. This means:
- If subtracting would result in a negative number, it returns 0 instead
- For example: `1.saturating_sub(2)` returns 0, not -1
- This is perfect for list navigation where we can't go below the first item

Here's the keyboard handling code:

```rust
if let Some(selected_item) = self.selected_item {
    if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
        self.selected_item = Some((selected_item + 1).min(items_len - 1));
    }
    if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
        self.selected_item = Some(selected_item.saturating_sub(1));
    }
    if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
        self.item_open[selected_item] = !self.item_open[selected_item];
    }
}
```

## Drawing the List

Each item is drawn as a collapsible header with sub-items. Selected items get highlighted:

```rust
for i in 0..items_len {
    let open = self.item_open[i];
    ui.collapsing(format!("{}", self.items[i]), |ui| {
        if Some(i) == self.selected_item {
            ui.visuals_mut().selection.bg_fill = egui::Color32::from_gray(196);
        }
        ui.label(format!("Sub-item {}-1", i + 1));
        ui.label(format!("Sub-item {}-2", i + 1));
        ui.label(format!("Sub-item {}-3", i + 1));
    });
    self.item_open[i] = open;
}
```

## Using the Widget

Create a new list with your items:

```rust
let list = SelectableList::new(vec![
    "Item 1".to_string(),
    "Item 2".to_string(),
    "Item 3".to_string(),
]);
```

Draw it in your UI:

```rust
list.show(ctx, ui);
```

That's it. You now have a reusable list widget with keyboard navigation and collapsible items.


## Minimum Working Example

/// tip | Adapted from Egui Hello World
This is adapted from the Hello World Example
///

<details closed markdown><summary>


Cargo.toml
</summary>

```toml

[package]
name = "egui_list"
version = "0.1.0"
authors = ["Emil Ernerfeldt <emil.ernerfeldt@gmail.com>"]
license = "MIT OR Apache-2.0"
edition = "2021"
publish = false

[dependencies]
eframe = { version = "0.30.0", features = ["__screenshot"] }
egui_extras = { version = "0.30.0", features = ["default", "image"] }
env_logger = {version= "0.11.6", default-features = false, features = [
    "auto-color",
    "humantime",
] }
```


</details>

```rust
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

struct SelectableList<T> {
    items: Vec<T>,
    selected_item: Option<usize>,
    item_open: Vec<bool>,
}

impl<T: std::fmt::Display> SelectableList<T> {
    fn new(items: Vec<T>) -> Self {
        let len = items.len();
        Self {
            items,
            selected_item: None,
            item_open: vec![false; len],
        }
    }

    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let items_len = self.items.len();

        // Handle keyboard input
        if let Some(selected_item) = self.selected_item {
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                self.selected_item = Some((selected_item + 1).min(items_len - 1));
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                self.selected_item = Some(selected_item.saturating_sub(1));
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                self.item_open[selected_item] = !self.item_open[selected_item];
            }
        }

        for i in 0..items_len {
            let open = self.item_open[i];
            ui.collapsing(format!("{}", self.items[i]), |ui| {
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
    list: SelectableList<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            list: SelectableList::new(
                [
                    "Item 1".to_string(),
                    "Item 2".to_string(),
                    "Item 3".to_string(),
                    "Item 4".to_string(),
                    "Item 5".to_string(),
                ]
                .to_vec(),
            ),
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
        });
    }
}

```

