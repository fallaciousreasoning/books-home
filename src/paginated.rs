use eframe::egui::{Ui};

pub struct GridConfig {
    
}

type RenderItem<T> = fn(&mut Ui, &T) -> ();

pub fn paginated<T>(ui: &mut Ui, items: Vec<T>, render_item: RenderItem<T>) {
    for item in items {
        render_item(ui, &item);
    }
}