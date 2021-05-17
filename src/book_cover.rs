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
    ui.painter().rect(rect, visuals.corner_radius, visuals.bg_fill, visuals.bg_stroke);
    ui.painter().rect_filled(
        egui::Rect::from_min_size(
            rect.left_top(), 
            egui::vec2(width*book.progress, 10.)),
        visuals.corner_radius,
        egui::Color32::BLACK);
    ui.label(book.title);
}