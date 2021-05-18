use eframe::egui::{self};

pub struct BookDetails {
    pub title: String,
    pub progress: f32,
}

pub fn book_cover(ui: &mut egui::Ui, book: BookDetails) {
    let aspect_ratio = 2. / 3.;
    let height = 200.;
    let width = aspect_ratio * height;
    let desired_size = egui::vec2(width, height);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let visuals = ui.style().interact(&response);
    let corner_radius = visuals.corner_radius;
    let bg_fill = visuals.bg_fill;
    let bg_stroke = visuals.bg_stroke;

    ui.allocate_ui_at_rect(rect, |ui| {
        ui.painter().rect(rect, corner_radius, bg_fill, bg_stroke);
        let layout = egui::Layout::top_down(egui::Align::Min);
        layout.align_size_within_rect(rect.size(), rect);
        ui.with_layout(layout, |ui| {
            let (_, progress_rect) = ui.allocate_space(egui::vec2(width, 10.));
            ui.painter()
                .rect_filled(
                    egui::Rect::from_min_size(progress_rect.min, egui::vec2(width*book.progress, progress_rect.height())), 
                    corner_radius, 
                    egui::Color32::BLACK);
            ui.allocate_ui_at_rect(rect.shrink(ui.spacing().item_spacing.x), |ui| {
                ui.add_space(ui.spacing().item_spacing.y);
                ui.label(book.title);
            });
        });
    });
}
