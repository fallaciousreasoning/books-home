use eframe::egui::{pos2, Rect, Sense, Ui, Vec2};

pub struct GridConfig {}

type RenderItem<T> = fn(&mut Ui, &T) -> ();

pub fn paginated<T>(ui: &mut Ui, item_size: Vec2, items: Vec<T>, render_item: RenderItem<T>) {
    let top_left = ui.available_rect_before_wrap().min;
    let space = ui.available_size();

    let spacing = ui.spacing().item_spacing.x;
    let width_in_tiles = (space.x / (item_size.x + spacing)).floor() as i32;
    let height_in_tiles = (space.y / (item_size.x + spacing)).floor() as i32;

    let mut iter = items.iter();
    for row in 0..height_in_tiles {
        for column in 0..width_in_tiles {
            let item = match iter.next() {
                Some(item) => item,
                None => break,
            };

            let start_pos = pos2(
                top_left.x + column as f32 * (item_size.x + spacing),
                top_left.y + row as f32 * (item_size.y + spacing),
            );
            let rect = Rect::from_min_size(start_pos, item_size);
            ui.allocate_ui_at_rect(rect, |ui| {
                render_item(ui, &item);
            });
        }
    }
}
