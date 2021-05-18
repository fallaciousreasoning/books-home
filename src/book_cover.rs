use eframe::egui::{self};

pub struct BookDetails {
    pub title: String,
    pub progress: f32,
}

pub fn book_cover(ui: &mut egui::Ui, book: BookDetails) {
    let aspect_ratio = 2./3.;
    let height = 200.;
    let width = aspect_ratio * height;
    let desired_size = egui::vec2(width, height);
    
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let visuals = ui.style().interact(&response);
    let expansion = visuals.expansion;
    let rect = rect.expand(expansion);
    let corner_radius = visuals.corner_radius;
    ui.painter().rect(rect, corner_radius, visuals.bg_fill, visuals.bg_stroke);
    ui.allocate_ui_at_rect(rect, |ui| {
        let layout = egui::Layout::bottom_up(egui::Align::Min);
        ui.with_layout(layout, |ui| {
            let (_, progress_rect) = ui.allocate_space(egui::vec2(width*book.progress, 10.));
            ui.painter().rect_filled(
                progress_rect,
                corner_radius,
                egui::Color32::BLACK);
        });
        
        ui.allocate_ui_at_rect(rect.shrink(5.), |ui| {
            ui.label(book.title);
        });
    });
}