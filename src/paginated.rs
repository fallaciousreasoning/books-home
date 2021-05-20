use std::cmp::{max, min};

use eframe::egui::{Align, Rect, Sense, Ui, Vec2, pos2, vec2};

pub enum Alignment {
    Start,
    End,
    SpaceAround,
    SpaceBetween
}

pub struct GridConfig {
    align: Alignment,
    item_size: Vec2,
}

type RenderItem<T> = fn(&mut Ui, &T) -> ();

pub fn paginated<T>(ui: &mut Ui, item_size: Vec2, items: Vec<T>, render_item: RenderItem<T>) {
    let top_left = ui.available_rect_before_wrap().min;
    let space = ui.available_size();

    let spacing = ui.spacing().item_spacing.x;
    let width_in_tiles = max((space.x / (item_size.x + spacing)).floor() as i32, 1);
    let height_in_tiles = max((space.y / (item_size.x + spacing)).floor() as i32, 1);

    let align = Alignment::SpaceAround;

    let mut iter = items.iter();
    for row in 0..height_in_tiles {
        for column in 0..width_in_tiles {
            let items_this_row = min(width_in_tiles, items.len() as i32 - row * width_in_tiles);
            let mut top_left = top_left;
            let mut spacing = vec2(spacing, spacing);
            let remaining_x_space = space.x - (items_this_row as f32 * (spacing.x + item_size.x));

            match align {
                Alignment::Start => {}
                Alignment::End => top_left.x += remaining_x_space,
                Alignment::SpaceBetween => {
                    spacing.x += remaining_x_space/items_this_row as f32;
                    top_left.x += spacing.x/2.;
                },
                Alignment::SpaceAround => top_left.x += remaining_x_space /2.
            };

            let item = match iter.next() {
                Some(item) => item,
                None => break,
            };

            let start_pos = pos2(
                top_left.x + column as f32 * (item_size.x + spacing.x),
                top_left.y + row as f32 * (item_size.y + spacing.y),
            );
            let rect = Rect::from_min_size(start_pos, item_size);
            ui.allocate_ui_at_rect(rect, |ui| {
                render_item(ui, &item);
            });
        }
    }
}
