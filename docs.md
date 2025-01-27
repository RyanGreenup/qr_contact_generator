# Building a Simple List Widget in Egui

This tutorial shows how to build a list widget with keyboard navigation and collapsible items in Egui.

## The Core Structure

The list widget needs three pieces of state:
- The items to display
- Which item is selected 
- Which items are expanded/collapsed

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
