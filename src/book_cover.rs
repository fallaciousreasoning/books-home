use eframe::egui;
use egui::Response;

pub struct BookDetails {
    pub title: String,
    pub progress: f32,
}

pub fn book_cover(ui: &mut egui::Ui, book: BookDetails) {
    let aspect_ratio = 2./3.;
    let height = 200.;
    let width = aspect_ratio * height;
    let desired_size = egui::vec2(width, height);
    
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let visuals = ui.style().interact(&response);
    let rect = rect.expand(visuals.expansion);
    let corner_radius = visuals.corner_radius;
    ui.painter().rect(rect, corner_radius, visuals.bg_fill, visuals.bg_stroke);
    ui.allocate_ui_at_rect(rect, |ui| {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            let (_, progress_rect) = ui.allocate_space(egui::vec2(width, 10.));
            ui.painter().rect_filled(
                progress_rect,
                corner_radius,
                egui::Color32::BLACK);
            ui.label(book.title);
        });
    });
}